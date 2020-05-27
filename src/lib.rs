#[macro_use]
extern crate yaserde_derive;

use std::io::prelude::*;
use xml_schema_derive::XmlSchema;
use yaserde::{YaDeserialize, YaSerialize};

pub mod ttml1 {
  use super::*;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-animation.xsd",
    target_prefix = "ttml",
    module_namespace_mapping = "http://www.w3.org/ns/ttml: crate::ttml1"
  )]
  struct Ttml1AnimationSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-content.xsd",
    target_prefix = "ttml",
    module_namespace_mapping = "http://www.w3.org/ns/ttml: crate::ttml1"
  )]
  struct Ttml1ContentSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-core-attribs.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1CoreAttribsSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-datatypes.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1DatatypesSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-document.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1DocumentSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-head.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1HeadSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-layout.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1LayoutSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-metadata-attribs.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1MetadataAttribsSchema;

  pub mod metadata_items {
    use super::*;

    #[derive(Debug, XmlSchema)]
    #[xml_schema(
      source = "w3c/ttml1/spec/xsd/ttml1-metadata-items.xsd",
      target_prefix = "ttml",
    )]
    struct Ttml1MetadataItemsSchema;
  }

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-metadata.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1MetadataSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-parameter-attribs.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1ParameterAttributesSchema;

  pub mod parameters_items {
    use super::*;

    #[derive(Debug, XmlSchema)]
    #[xml_schema(
      source = "w3c/ttml1/spec/xsd/ttml1-parameter-items.xsd",
      target_prefix = "ttml",
    )]
    struct Ttml1ParameterItemsSchema;
  }

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-parameters.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1ParametersSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-profile.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1ProfileSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-styling-attribs.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1StylingAttributesSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-styling.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1StylingSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1-timing-attribs.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1TimingAttributesSchema;

  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "w3c/ttml1/spec/xsd/ttml1.xsd",
    target_prefix = "ttml",
  )]
  struct Ttml1Schema;
}
