// pub struct NetworkStrategy {
//     encoder: Encoder,
//     connection_pool: ConnectionPool,
//     config: Arc<Config>,
//     stats: Arc<Collector>,
// }

// impl<T: Sendable> TransportStrategy<T> for NetworkStrategy {
//     fn send(&mut self, item: T) -> Result<()> {
//         self.encoder.cache(item);
//         if self.encoder.should_flush() {
//             self.flush()?;
//         }
//         Ok(())
//     }

//     fn flush(&mut self) -> Result<()> {
//         let conn = self.connection_pool.get_connection()?;
//         conn.send(self.encoder.take_buffer())?;
//         Ok(())
//     }
// }
