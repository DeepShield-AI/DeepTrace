use futures::Stream;
use futures_util::TryStreamExt;
use log::error;
use observ_runtime::spawn_blocking;
use std::{io, path::Path, str::FromStr};
use tokio::{
	fs::File,
	io::{AsyncBufReadExt, BufReader},
	task::JoinError,
};
use tokio_stream::{StreamExt, wrappers::LinesStream};

fn join_err_to_io(e: JoinError) -> io::Error {
	io::Error::other(e)
}

pub async fn path_exists<T>(path: T) -> io::Result<bool>
where
	T: AsRef<Path> + Send + 'static,
{
	let path = path.as_ref().to_owned();
	spawn_blocking(move || path.exists()).await.map_err(join_err_to_io)
}

pub async fn path_exists_lossy<T>(path: T) -> bool
where
	T: AsRef<Path> + Send + 'static,
{
	path_exists(path).await.unwrap_or(false)
}

pub async fn read_to_string<P>(path: P) -> io::Result<String>
where
	P: AsRef<Path> + Send + 'static,
{
	let path = path.as_ref().to_owned();

	spawn_blocking(move || std::fs::read_to_string(&path))
		.await
		.map_err(join_err_to_io)?
}

pub async fn read_into<T, R, E>(path: T) -> Result<R, E>
where
	T: AsRef<Path> + Send + 'static,
	R: FromStr + Send + 'static,
	E: From<io::Error> + From<<R as FromStr>::Err> + Send + 'static,
{
	let contents = read_to_string(path).await?;
	R::from_str(contents.trim()).map_err(Into::into)
}

pub async fn read_lines<T>(path: T) -> io::Result<impl Stream<Item = io::Result<String>> + Send>
where
	T: AsRef<Path> + Send + 'static,
{
	let file = File::open(path).await?;
	let reader = BufReader::new(file);
	Ok(LinesStream::new(reader.lines()))
}

pub async fn read_lines_into<T, R, E>(
	path: T,
) -> io::Result<impl Stream<Item = Result<R, E>> + Send>
where
	T: AsRef<Path> + Send + 'static,
	R: FromStr + Send + 'static,
	E: From<io::Error> + From<<R as FromStr>::Err> + Send + 'static,
{
	let stream = read_lines(path).await?;
	let stream = stream
		.map_err(E::from)
		.and_then(|line| async move { R::from_str(line.trim()).map_err(E::from) });
	Ok(stream)
}

pub async fn read_first_line<T>(path: T) -> io::Result<String>
where
	T: AsRef<Path> + Send + 'static,
{
	let mut s = read_lines(path).await?;
	match s.next().await {
		Some(Ok(line)) => Ok(line),
		Some(Err(e)) => Err(e),
		None => Err(io::Error::new(io::ErrorKind::UnexpectedEof, "file is empty")),
	}
}

pub fn filter_result<T, E>(result: Result<T, E>, message: &'static str) -> Option<T>
where
	E: std::error::Error,
{
	result.map_err(|error| error!("{message}: {error}")).ok()
}

fn next_or<E, It>(it: &mut It) -> Result<It::Item, E>
where
	It: Iterator,
	E: From<io::Error>,
{
	it.next()
		.ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))
		.map_err(E::from)
}

pub fn parse_next<R, E, O, It, I>(it: &mut It) -> Result<R, E>
where
	It: Iterator<Item = I>,
	I: AsRef<str>,
	R: FromStr<Err = O>,
	E: From<io::Error> + From<O>,
{
	let v = next_or::<E, _>(it)?;
	R::from_str(v.as_ref()).map_err(E::from)
}
