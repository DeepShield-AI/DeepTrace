use crate::{attr, types, utils};
use syn::{Data, DeriveInput, Error, Fields, Ident, Result, Type, Visibility};

pub struct Struct<'a> {
	pub ident: &'a Ident,
	pub attrs: attr::structs::Attrs,
	pub fields: Vec<Field<'a>>,
	pub visibility: &'a Visibility,
}

pub struct Field<'a> {
	pub attrs: attr::field::Attrs,
	pub member: MemberUnraw,
	pub ty: &'a Type,
	pub visibility: &'a Visibility,
	pub is_optional: bool,
	pub inner_type: Option<Type>,
	pub uom_conversion_code: Option<proc_macro2::TokenStream>,
}

#[derive(Clone)]
pub enum MemberUnraw {
	Named(IdentUnraw),
	Unnamed,
}

#[derive(Clone)]
pub struct IdentUnraw {
	pub raw: Ident,
}

impl<'a> Struct<'a> {
	pub fn from_syn(node: &'a DeriveInput) -> Result<Self> {
		let data = match &node.data {
			Data::Struct(data) => data,
			_ => return Err(Error::new_spanned(node, "only structs are supported")),
		};
		let attrs = attr::get_struct_attrs(&node.attrs)?;
		let fields = Field::from_fields(&data.fields)?;

		Ok(Self { attrs, ident: &node.ident, fields, visibility: &node.vis })
	}
}

impl<'a> Field<'a> {
	fn from_fields(fields: &'a Fields) -> Result<Vec<Self>> {
		fields.iter().map(Field::from_syn).collect()
	}

	fn from_syn(node: &'a syn::Field) -> Result<Self> {
		let attrs = attr::get_field_attrs(&node.attrs)?;
		let inner_type = utils::extract_option_inner_type(&node.ty);
		let is_optional = inner_type.is_some();

		let unit_path = attrs.unit.as_ref();

		let uom_conversion_code = if let Some(ref inner) = inner_type {
			// For Option<T>, analyze the inner type T with custom unit if specified
			types::generate_uom_conversion_code_with_unit(inner, unit_path)
		} else {
			// For direct types, analyze the type itself with custom unit if specified
			types::generate_uom_conversion_code_with_unit(&node.ty, unit_path)
		};

		Ok(Self {
			attrs,
			member: match &node.ident {
				Some(ident) => MemberUnraw::Named(IdentUnraw { raw: ident.clone() }),
				None => MemberUnraw::Unnamed,
			},
			ty: &node.ty,
			visibility: &node.vis,
			is_optional,
			inner_type,
			uom_conversion_code,
		})
	}
}
