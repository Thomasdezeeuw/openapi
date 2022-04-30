//! Open API Specification
//!
//! Currently this only supports [OpenAPI Specification v3.1.0].
//!
//! The main type is  [`Spec`], which represents an OpenAPI specification.
//!
//! [OpenAPI Specification v3.1.0]: https://spec.openapis.org/oas/v3.1.0.html

// Implements:
// OpenAPI                 version 3.1.0
// JSON Schema             draft-bhutton-json-schema-00
// JSON Schema Validation  draft-bhutton-json-schema-validation-00

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod parse;
#[cfg(any(feature = "json", feature = "yaml"))]
pub use parse::read_from_file;
#[cfg(feature = "json")]
pub use parse::read_from_json_file;
#[cfg(feature = "yaml")]
pub use parse::read_from_yaml_file;

/// This is the root object of the OpenAPI document.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    /// This string MUST be the version number of the OpenAPI Specification that
    /// the OpenAPI document uses. The `openapi` field SHOULD be used by tooling
    /// to interpret the OpenAPI document. This is *not* related to the API
    /// [`Info::version`] string.
    pub openapi: Version,
    /// Provides metadata about the API. The metadata MAY be used by tooling as
    /// required.
    pub info: Info,
    /// The default value for the `$schema` keyword within [Schema Objects]
    /// contained within this OAS document. This MUST be in the form of a URI.
    ///
    /// [Schema Objects]: Schema
    #[serde(default)]
    pub json_schema_dialect: Option<String>,
    /// An array of Server Objects, which provide connectivity information to a
    /// target server. If the `servers` property is not provided, or is an empty
    /// array, the default value would be a [Server Object] with a [url] value
    /// of `/`.
    ///
    /// [Server Object]: Server
    /// [url]: Server::url
    #[serde(default)]
    pub servers: Vec<Server>,
    /// The available paths and operations for the API.
    #[serde(default)]
    pub paths: Paths,
    /// The incoming webhooks that MAY be received as part of this API and that
    /// the API consumer MAY choose to implement. Closely related to the
    /// `callbacks` feature, this section describes requests initiated other
    /// than by an API call, for example by an out of band registration. The key
    /// name is a unique string to refer to each webhook, while the (optionally
    /// referenced) Path Item Object describes a request that may be initiated
    /// by the API provider and the expected responses.
    #[serde(default)]
    pub webhooks: HashMap<String, PathItem>, // NOTE: `PathItem` includes all fields of `Reference`.
    /// An element to hold various schemas for the document.
    #[serde(default)]
    pub components: Components,
    /// A declaration of which security mechanisms can be used across the API.
    /// The list of values includes alternative security requirement objects
    /// that can be used. Only one of the security requirement objects need to
    /// be satisfied to authorize a request. Individual operations can override
    /// this definition. To make security optional, an empty security
    /// requirement (`{}`) can be included in the array.
    #[serde(default)]
    pub security: Vec<SecurityRequirement>,
    /// A list of tags used by the document with additional metadata. The order
    /// of the tags can be used to reflect on their order by the parsing tools.
    /// Not all tags that are used by the [Operation Object] must be declared.
    /// The tags that are not declared MAY be organized randomly or based on the
    /// tools' logic. Each tag name in the list MUST be unique.
    ///
    /// [Operation Object]: Operation
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Additional external documentation.
    #[serde(default)]
    pub external_docs: Option<ExternalDocument>,
}

/// The OpenAPI Specification version.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Version {
    /// Version `3.1.0`.
    #[serde(rename = "3.1.0")]
    OpenApi3_1,
}

/// The object provides metadata about the API.
///
/// The metadata MAY be used by the clients if needed, and MAY be presented in
/// editing or documentation generation tools for convenience.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// The title of the API.
    pub title: String,
    /// A short summary of the API.
    #[serde(default)]
    pub summary: Option<String>,
    /// A description of the API. [CommonMark syntax] MAY be used for rich text
    /// representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// A URL to the Terms of Service for the API. This MUST be in the form of a
    /// URL.
    #[serde(default)]
    pub terms_of_service: Option<String>,
    /// The contact information for the exposed API.
    #[serde(default)]
    pub contact: Option<Contact>,
    /// The license information for the exposed API.
    #[serde(default)]
    pub license: Option<License>,
    /// The version of the OpenAPI document (which is distinct from the OpenAPI
    /// Specification version or the API implementation version).
    pub version: String,
}

/// Contact information for the exposed API.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    #[serde(default)]
    pub name: Option<String>,
    /// The URL pointing to the contact information. This MUST be in the form of
    /// a URL.
    #[serde(default)]
    pub url: Option<String>,
    /// The email address of the contact person/organization. This MUST be in
    /// the form of an email address.
    #[serde(default)]
    pub email: Option<String>,
}

/// License information for the exposed API.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    /// The license name used for the API.
    pub name: String,
    /// An [SPDX] license expression for the API. The `identifier` field is
    /// mutually exclusive of the `url` field.
    ///
    /// [SPDX]: (https://spdx.org/spdx-specification-21-web-version#h.jxpfx0ykyb60)
    #[serde(default)]
    pub identifier: Option<String>,
    /// A URL to the license used for the API. This MUST be in the form of a
    /// URL. The `url` field is mutually exclusive of the `identifier` field.
    #[serde(default)]
    pub url: Option<String>,
}

/// An object representing a Server.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    /// A URL to the target host. This URL supports Server Variables and MAY be
    /// relative, to indicate that the host location is relative to the location
    /// where the OpenAPI document is being served. Variable substitutions will
    /// be made when a variable is named in `{`brackets`}`.
    pub url: String,
    /// An optional string describing the host designated by the URL.
    /// [CommonMark syntax] MAY be used for rich text representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// A map between a variable name and its value. The value is used for
    /// substitution in the server's URL template.
    #[serde(default)]
    pub variables: HashMap<String, ServerVariable>,
}

/// An object representing a Server Variable for server URL template
/// substitution.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerVariable {
    /// An enumeration of string values to be used if the substitution options
    /// are from a limited set. The array MUST NOT be empty.
    #[serde(default)]
    pub r#enum: Vec<String>,
    /// The default value to use for substitution, which SHALL be sent if an
    /// alternate value is _not_ supplied. Note this behavior is different than
    /// the [Schema Object's](#schemaObject) treatment of default values,
    /// because in those cases parameter values are optional. If the
    /// [`enum`](#serverVariableEnum) is defined, the value MUST exist in the
    /// enum's values.
    pub default: String,
    /// An optional description for the server variable. [CommonMark syntax] MAY
    /// be used for rich text representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
}

/// Holds a set of reusable objects for different aspects of the OAS.
///
/// All objects defined within the components object will have no effect on the
/// API unless they are explicitly referenced from properties outside the
/// components object.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    /// An object to hold reusable [Schema Objects].
    ///
    /// [Schema Objects]: Schema
    #[serde(default)]
    pub schemas: HashMap<String, Schema>,
    /// An object to hold reusable [Response Objects].
    ///
    /// [Response Objects]: Response
    #[serde(default)]
    pub responses: HashMap<String, Reference<Response>>,
    /// An object to hold reusable [Parameter Objects].
    ///
    /// [Parameter Objects]: Parameter
    #[serde(default)]
    pub parameters: HashMap<String, Reference<Parameter>>,
    /// An object to hold reusable [Example Objects].
    ///
    /// [Example Objects]: Example
    #[serde(default)]
    pub examples: HashMap<String, Reference<Example>>,
    /// An object to hold reusable [Request Body Objects].
    ///
    /// [Request Body Objects]: RequestBody
    #[serde(default)]
    pub request_bodies: HashMap<String, Reference<RequestBody>>,
    /// An object to hold reusable [Header Objects].
    ///
    /// [Header Objects]: Header
    #[serde(default)]
    pub headers: HashMap<String, Reference<Header>>,
    /// An object to hold reusable [Security Scheme Objects].
    ///
    /// [Security Scheme Objects]: SecurityScheme
    #[serde(default)]
    pub security_schemes: HashMap<String, Reference<SecurityScheme>>,
    /// An object to hold reusable [Link Objects].
    ///
    /// [Link Objects]: Link
    #[serde(default)]
    pub links: HashMap<String, Reference<Link>>,
    /// An object to hold reusable [Callback Objects].
    ///
    /// [Callback Objects]: Callback
    #[serde(default)]
    pub callbacks: HashMap<String, Reference<Callback>>,
    /// An object to hold reusable [Path Item Object].
    ///
    /// [Path Item Objects]: PathItem
    #[serde(default)]
    pub path_items: HashMap<String, PathItem>, // NOTE: `PathItem` includes all fields of `Reference`.
}

/// Holds the relative paths to the individual endpoints and their operations.
///
/// The path is appended to the URL from the [Server Object] in order to
/// construct the full URL. The Paths MAY be empty, due to Access Control List
/// (ACL) constraints.
///
/// A relative path to an individual endpoint. The field name MUST begin with a
/// forward slash (`/`). The path is **appended** (no relative URL resolution)
/// to the expanded URL from the [Server Object]'s `url` field in order to
/// construct the full URL. Path templating is allowed. When matching URLs,
/// concrete (non-templated) paths would be matched before their templated
/// counterparts. Templated paths with the same hierarchy but different
/// templated names MUST NOT exist as they are identical. In case of ambiguous
/// matching, it's up to the tooling to decide which one to use.
///
/// [Server Object]: Server
pub type Paths = HashMap<String, PathItem>;

/// Describes the operations available on a single path.
///
/// A Path Item MAY be empty, due to ACL constraints. The path itself is still
/// exposed to the documentation viewer but they will not know which operations
/// and parameters are available.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathItem {
    /// Allows for a referenced definition of this path item. The referenced
    /// structure MUST be in the form of a [Path Item Object]. In case a Path
    /// Item Object field appears both in the defined object and the referenced
    /// object, the behavior is undefined. See the rules for resolving Relative
    /// References.
    ///
    /// [Path Item Object]: PathItem
    #[serde(rename = "$ref", default)]
    pub r#ref: Option<String>,
    /// An optional, string summary, intended to apply to all operations in this
    /// path.
    #[serde(default)]
    pub summary: Option<String>,
    /// An optional, string description, intended to apply to all operations in
    /// this path. [CommonMark syntax] MAY be used for rich text representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// A definition of a GET operation on this path.
    #[serde(default)]
    pub get: Option<Operation>,
    /// A definition of a PUT operation on this path.
    #[serde(default)]
    pub put: Option<Operation>,
    /// A definition of a POST operation on this path.
    #[serde(default)]
    pub post: Option<Operation>,
    /// A definition of a DELETE operation on this path.
    #[serde(default)]
    pub delete: Option<Operation>,
    /// A definition of a OPTIONS operation on this path.
    #[serde(default)]
    pub options: Option<Operation>,
    /// A definition of a HEAD operation on this path.
    #[serde(default)]
    pub head: Option<Operation>,
    /// A definition of a PATCH operation on this path.
    #[serde(default)]
    pub patch: Option<Operation>,
    /// A definition of a TRACE operation on this path.
    #[serde(default)]
    pub trace: Option<Operation>,
    /// An alternative `server` array to service all operations in this path.
    #[serde(default)]
    pub servers: Vec<Server>,
    /// A list of parameters that are applicable for all the operations
    /// described under this path. These parameters can be overridden at the
    /// operation level, but cannot be removed there. The list MUST NOT include
    /// duplicated parameters. A unique parameter is defined by a combination of
    /// a [name] and [location] The list can use the [Reference
    /// Object](#referenceObject) to link to parameters that are defined at the
    /// [OpenAPI Object's components/parameters].
    ///
    /// [name]: Parameter::name
    /// [location]: Parameter::in
    /// [OpenAPI Object's components/parameters]: Components::parameters
    #[serde(default)]
    pub parameters: Vec<Reference<Parameter>>,
}

/// Describes a single API operation on a path.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    /// A list of tags for API documentation control. Tags can be used for
    /// logical grouping of operations by resources or any other qualifier.
    #[serde(default)]
    pub tags: Vec<String>,
    /// A short summary of what the operation does.
    #[serde(default)]
    pub summary: Option<String>,
    /// A verbose explanation of the operation behavior. [CommonMark syntax] MAY
    /// be used for rich text representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// Additional external documentation for this operation.
    #[serde(default)]
    pub external_docs: Option<ExternalDocument>,
    /// Unique string used to identify the operation. The id MUST be unique
    /// among all operations described in the API. The operationId value is
    /// **case-sensitive**. Tools and libraries MAY use the operationId to
    /// uniquely identify an operation, therefore, it is RECOMMENDED to follow
    /// common programming naming conventions.
    #[serde(default)]
    pub operation_id: Option<String>,
    /// A list of parameters that are applicable for this operation. If a
    /// parameter is already defined at the [Path Item], the new definition will
    /// override it but can never remove it. The list MUST NOT include
    /// duplicated parameters. A unique parameter is defined by a combination of
    /// a [name] and [location]. The list can use the [Reference Object] to link
    /// to parameters that are defined at the [OpenAPI Object's
    /// components/parameters].
    ///
    /// [Path Item]: PathItem::parameters
    /// [name]: Parameter::name
    /// [location]: Parameter::in
    /// [Reference Object]: Reference
    /// [OpenAPI Object's components/parameters]: Components::parameters
    #[serde(default)]
    pub parameters: Vec<Reference<Parameter>>,
    /// The request body applicable for this operation. The `request_body` is
    /// fully supported in HTTP methods where the HTTP 1.1 specification
    /// [RFC7231] has explicitly defined semantics for request bodies. In other
    /// cases where the HTTP spec is vague (such as [GET], [HEAD] and [DELETE]),
    /// `requestBody` is permitted but does not have well-defined semantics and
    /// SHOULD be avoided if possible.
    ///
    /// [RFC7231]: https://tools.ietf.org/html/rfc7231#section-4.3.1
    /// [GET]: https://tools.ietf.org/html/rfc7231#section-4.3.1
    /// [HEAD]: https://tools.ietf.org/html/rfc7231#section-4.3.2
    /// [DELETE]: https://tools.ietf.org/html/rfc7231#section-4.3.5
    #[serde(default)]
    pub request_body: Option<Reference<RequestBody>>,
    /// The list of possible responses as they are returned from executing this
    /// operation.
    #[serde(default)]
    pub responses: Option<Responses>,
    /// A map of possible out-of band callbacks related to the parent operation.
    /// The key is a unique identifier for the Callback Object. Each value in
    /// the map is a [Callback Object] that describes a request that may be
    /// initiated by the API provider and the expected responses.
    ///
    /// [Callback Object]: Callback
    #[serde(default)]
    pub callbacks: HashMap<String, Reference<Callback>>,
    /// Declares this operation to be deprecated. Consumers SHOULD refrain from
    /// usage of the declared operation. Default value is `false`.
    #[serde(default)]
    pub deprecated: bool,
    /// A declaration of which security mechanisms can be used for this
    /// operation. The list of values includes alternative security requirement
    /// objects that can be used. Only one of the security requirement objects
    /// need to be satisfied to authorize a request. To make security optional,
    /// an empty security requirement (`{}`) can be included in the array. This
    /// definition overrides any declared top-level [`security`]. To remove a
    /// top-level security declaration, an empty array can be used.
    ///
    /// [`security`]: Spec::security
    #[serde(default)]
    pub security: Vec<SecurityRequirement>,
    /// An alternative `server` array to service this operation. If an
    /// alternative `server` object is specified at the [Path Item Object] or
    /// [Root] level, it will be overridden by this value.
    ///
    /// [Path Item Object]: PathItem
    /// [Root]: Spec
    #[serde(default)]
    pub servers: Vec<Server>,
}

/// Allows referencing an external resource for extended documentation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocument {
    /// A description of the target documentation. [CommonMark syntax] MAY be
    /// used for rich text representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// The URL for the target documentation. This MUST be in the form of a URL.
    pub url: String,
}

/// Describes a single operation parameter.
///
/// A unique parameter is defined by a combination of a [name] and [location].
///
/// [name]: Parameter::name
/// [location]: Parameter::in
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    /// The name of the parameter. Parameter names are *case sensitive*.
    ///
    ///  * If [`in`] is `"path"`, the `name` field MUST correspond to a template
    ///    expression occurring within the [path] field in the [Paths Object].
    ///    See Path Templating for further information.
    ///  * If [`in`] is `"header"` and the `name` field is `"Accept"`,
    ///    `"Content-Type"` or `"Authorization"`, the parameter definition SHALL
    ///    be ignored.
    ///  * For all other cases, the `name` corresponds to the parameter name
    ///    used by the [`in`] property.
    ///
    /// [`in`]: Parameter::in
    /// [path]: Paths
    /// [Paths Object]: Paths
    pub name: String,
    /// The location of the parameter.
    pub r#in: ParameterLocation,
    /// A brief description of the parameter. This could contain examples of
    /// use. [CommonMark syntax] MAY be used for rich text representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// Determines whether this parameter is mandatory. If the [parameter
    /// location] is `"path"`, this property is **REQUIRED** and its value MUST
    /// be `true`. Otherwise, the property MAY be included and its default value
    /// is `false`.
    ///
    /// [parameter location]: Parameter::in
    #[serde(default)]
    pub required: bool,
    /// Specifies that a parameter is deprecated and SHOULD be transitioned out
    /// of usage. Default value is `false`.
    #[serde(default)]
    pub deprecated: bool,
    /// Sets the ability to pass empty-valued parameters. This is valid only for
    /// `query` parameters and allows sending a parameter with an empty value.
    /// Default value is `false`. If [`style`] is used, and if behavior is `n/a`
    /// (cannot be serialized), the value of `allow_empty_value` SHALL be
    /// ignored. Use of this property is NOT RECOMMENDED, as it is likely to be
    /// removed in a later revision.
    ///
    /// [`style`]: Parameter::style
    #[serde(default)]
    pub allow_empty_value: bool,
    /// Describes how the parameter value will be serialized depending on the
    /// type of the parameter value.
    ///
    /// Default values (based on value of `in`):
    ///  * for `query` - `form`
    ///  * for `path` - `simple`
    ///  * for `header` - `simple`
    ///  * for `cookie` - `form`
    #[serde(default)]
    pub style: Option<ParameterStyle>,
    /// When this is true, parameter values of type `array` or `object` generate
    /// separate parameters for each value of the array or key-value pair of the
    /// map. For other types of parameters this property has no effect. When
    /// [`style`] is `form`, the default value is `true`. For all other styles,
    /// the default value is `false`.
    ///
    /// [`style`]: Parameter::style
    #[serde(default)]
    pub explode: bool,
    /// Determines whether the parameter value SHOULD allow reserved characters,
    /// as defined by [RFC3986] `:/?#[]@!$&'()*+,;=` to be included without
    /// percent-encoding. This property only applies to parameters with an `in`
    /// value of `query`. The default value is `false`.
    ///
    /// [RFC3986]: (https://tools.ietf.org/html/rfc3986#section-2.2)
    #[serde(default)]
    pub allow_reserved: bool,
    /// The schema defining the type used for the parameter.
    #[serde(default)]
    pub schema: Option<Schema>,
    /// Example of the parameter's potential value. The example SHOULD match the
    /// specified schema and encoding properties if present. The `example` field
    /// is mutually exclusive of the `examples` field. Furthermore, if
    /// referencing a `schema` that contains an example, the `example` value
    /// SHALL _override_ the example provided by the schema. To represent
    /// examples of media types that cannot naturally be represented in JSON or
    /// YAML, a string value can contain the example with escaping where
    /// necessary.
    #[serde(default)]
    pub example: Option<Any>,
    /// Examples of the parameter's potential value. Each example SHOULD contain
    /// a value in the correct format as specified in the parameter encoding.
    /// The `examples` field is mutually exclusive of the `example` field.
    /// Furthermore, if referencing a `schema` that contains an example, the
    /// `examples` value SHALL _override_ the example provided by the schema.
    #[serde(default)]
    pub examples: HashMap<String, Reference<Example>>,
    /// A map containing the representations for the parameter. The key is the
    /// media type and the value describes it. The map MUST only contain one
    /// entry.
    #[serde(default)]
    pub content: HashMap<String, MediaType>,
}

/// There are four possible parameter locations specified by the
/// [`Parameter::in`] field.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParameterLocation {
    /// Used together with Path Templating, where the parameter value is
    /// actually part of the operation's URL. This does not include the host or
    /// base path of the API. For example, in `/items/{itemId}`, the path
    /// parameter is `itemId`.
    Path,
    /// Parameters that are appended to the URL. For example, in
    /// `/items?id=###`, the query parameter is `id`.
    Query,
    /// Custom headers that are expected as part of the request. Note that
    /// [RFC7230] states header names are case insensitive.
    ///
    /// [RFC7230]: https://tools.ietf.org/html/rfc7230#page-22
    Header,
    /// Used to pass a specific cookie value to the API.
    Cookie,
}

/// Parameter style.
///
/// See [`Parameter::style`].
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParameterStyle {
    /// Path-style parameters defined by [RFC6570].
    ///
    /// type: `primitive`, `array`, `object`.
    /// in: `path`
    ///
    /// [RFC6570]: https://tools.ietf.org/html/rfc6570#section-3.2.2
    Matrix,
    /// Label style parameters defined by [RFC6570].
    ///
    /// type: `primitive`, `array`, `object`
    /// in: `path`
    ///
    /// [RFC6570]: https://tools.ietf.org/html/rfc6570#section-3.2.2
    Label,
    /// Form style parameters defined by [RFC6570]. This option replaces
    /// `collectionFormat` with a `csv` (when `explode` is false) or `multi`
    /// (when `explode` is true) value from OpenAPI 2.0.
    ///
    /// type: `primitive`, `array`, `object`
    /// in: `query`, `cookie`
    ///
    /// [RFC6570]: https://tools.ietf.org/html/rfc6570#section-3.2.2
    Form,
    /// Simple style parameters defined by [RFC6570]. This option replaces
    /// `collectionFormat` with a `csv` value from OpenAPI 2.0.
    ///
    /// type: `array`
    /// in: `path`, `header`
    ///
    /// [RFC6570]: https://tools.ietf.org/html/rfc6570#section-3.2.2
    Simple,
    /// Space separated array or object values. This option replaces
    /// `collectionFormat` equal to `ssv` from OpenAPI 2.0.
    ///
    /// type: `array`, `object`
    /// in: `query`
    SpaceDelimited,
    /// Pipe separated array or object values. This option replaces
    /// `collectionFormat` equal to `pipes` from OpenAPI 2.0.
    ///
    /// type: `array`, `object`
    /// in: `query`
    PipeDelimited,
    /// Provides a simple way of rendering nested objects using form parameters.
    ///
    /// type: `object`
    /// in: `query`
    DeepObject,
}

/// Describes a single request body.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    /// A brief description of the request body. This could contain examples of
    /// use. [CommonMark syntax] MAY be used for rich text representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// The content of the request body. The key is a media type or [media type
    /// range] and the value describes it. For requests that match multiple
    /// keys, only the most specific key is applicable. e.g. `text/plain`
    /// overrides `text/*`.
    ///
    /// [media type range]: https://tools.ietf.org/html/rfc7231#appendix-D
    pub content: HashMap<String, MediaType>,
    /// Determines if the request body is required in the request. Defaults to
    /// `false`.
    #[serde(default)]
    pub required: bool,
}

/// Each Media Type Object provides schema and examples for the media type
/// identified by its key.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaType {
    /// The schema defining the content of the request, response, or parameter.
    #[serde(default)]
    pub schema: Option<Schema>,
    /// Example of the media type. The example object SHOULD be in the correct
    /// format as specified by the media type. The `example` field is mutually
    /// exclusive of the `examples` field. Furthermore, if referencing a
    /// `schema` which contains an example, the `example` value SHALL _override_
    /// the example provided by the schema.
    #[serde(default)]
    pub example: Option<Any>,
    /// Examples of the media type. Each example object SHOULD match the media
    /// type and specified schema if present. The `examples` field is mutually
    /// exclusive of the `example` field. Furthermore, if referencing a `schema`
    /// which contains an example, the `examples` value SHALL _override_ the
    /// example provided by the schema.
    #[serde(default)]
    pub examples: HashMap<String, Reference<Example>>,
    /// A map between a property name and its encoding information. The key,
    /// being the property name, MUST exist in the schema as a property. The
    /// encoding object SHALL only apply to `requestBody` objects when the media
    /// type is `multipart` or `application/x-www-form-urlencoded`.
    #[serde(default)]
    pub encoding: HashMap<String, Encoding>,
}

/// A single encoding definition applied to a single schema property.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encoding {
    /// The Content-Type for encoding a specific property. Default value depends
    /// on the property type:
    ///  * for `object` - `application/json`
    ///  * for `array` – the default is defined based on the inner type
    ///  * for all other cases the default is `application/octet-stream`.
    ///
    /// The value can be a specific media type (e.g. `application/json`), a
    /// wildcard media type (e.g. `image/*`), or a comma-separated list of the
    /// two types.
    #[serde(default)]
    pub content_type: Option<String>,
    /// A map allowing additional information to be provided as headers, for
    /// example `Content-Disposition`. `Content-Type` is described separately
    /// and SHALL be ignored in this section. This property SHALL be ignored if
    /// the request body media type is not a `multipart`.
    #[serde(default)]
    pub headers: HashMap<String, Reference<Header>>,
    /// Describes how a specific property value will be serialized depending on
    /// its type. See [Parameter Object] for details on the [`style`] property.
    /// The behavior follows the same values as `query` parameters, including
    /// default values. This property SHALL be ignored if the request body media
    /// type is not `application/x-www-form-urlencoded` or
    /// `multipart/form-data`. If a value is explicitly defined, then the value
    /// of [`content_type`] (implicit or explicit) SHALL be ignored.
    ///
    /// [Parameter Object]: Parameter
    /// [`style`]: Parameter::style
    /// [`content_type`]: Encoding::content_type
    #[serde(default)]
    pub style: Option<ParameterStyle>,
    /// When this is true, property values of type `array` or `object` generate
    /// separate parameters for each value of the array, or key-value-pair of
    /// the map. For other types of properties this property has no effect. When
    /// [`style`] is `form`, the default value is `true`. For all other styles,
    /// the default value is `false`. This property SHALL be ignored if the
    /// request body media type is not `application/x-www-form-urlencoded` or
    /// `multipart/form-data`. If a value is explicitly defined, then the value
    /// of [`content_type`] (implicit or explicit) SHALL be ignored.
    ///
    /// [`style`]: Encoding::style
    /// [`content_type`]: Encoding::content_type
    #[serde(default)]
    pub explode: bool,
    /// Determines whether the parameter value SHOULD allow reserved characters,
    /// as defined by [RFC3986] `:/?#[]@!$&'()*+,;=` to be included without
    /// percent-encoding. The default value is `false`. This property SHALL be
    /// ignored if the request body media type is not
    /// `application/x-www-form-urlencoded` or `multipart/form-data`. If a value
    /// is explicitly defined, then the value of [`content_type`] (implicit or
    /// explicit) SHALL be ignored.
    ///
    /// [RFC3986]: https://tools.ietf.org/html/rfc3986#section-2.2
    /// [`content_type`]: Encoding::content_type
    #[serde(default)]
    pub allow_reserved: bool,
}

/// A container for the expected responses of an operation.
///
/// The container maps a HTTP response code to the expected response.
///
/// The documentation is not necessarily expected to cover all possible HTTP
/// response codes because they may not be known in advance. However,
/// documentation is expected to cover a successful operation response and any
/// known errors.
///
/// The `default` MAY be used as a default response object for all HTTP codes
/// that are not covered individually by the `Responses Object`.
///
/// The `Responses Object` MUST contain at least one response code, and if only
/// one response code is provided it SHOULD be the response for a successful
/// operation call.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Responses {
    /// The documentation of responses other than the ones declared for specific
    /// HTTP response codes. Use this field to cover undeclared responses.
    #[serde(default)]
    pub default: Option<Reference<Response>>,
    /// Any HTTP status code can be used as the property name, but only one
    /// property per code, to describe the expected response for that HTTP
    /// status code.
    ///
    /// This field MUST be enclosed in quotation marks (for example, "200") for
    /// compatibility between JSON and YAML. To define a range of response
    /// codes, this field MAY contain the uppercase wildcard character `X`. For
    /// example, `2XX` represents all response codes between `[200-299]`. Only
    /// the following range definitions are allowed: `1XX`, `2XX`, `3XX`, `4XX`,
    /// and `5XX`. If a response is defined using an explicit code, the explicit
    /// code definition takes precedence over the range definition for that
    /// code.
    #[serde(flatten, default)]
    pub response: HashMap<String, Reference<Response>>,
}

/// Describes a single response from an API Operation, including design-time,
/// static `links` to operations based on the response.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// A description of the response. [CommonMark syntax] MAY be used for rich
    /// text representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    pub description: String,
    /// Maps a header name to its definition. [RFC7230] states header names are
    /// case insensitive. If a response header is defined with the name
    /// `"Content-Type"`, it SHALL be ignored.
    ///
    /// [RFC7230]: https://tools.ietf.org/html/rfc7230#page-22
    #[serde(default)]
    pub headers: HashMap<String, Reference<Header>>,
    /// A map containing descriptions of potential response payloads. The key is
    /// a media type or [media type range] and the value describes it. For
    /// responses that match multiple keys, only the most specific key is
    /// applicable. e.g. `text/plain` overrides `text/*`.
    ///
    /// [media type range]: https://tools.ietf.org/html/rfc7231#appendix-D
    #[serde(default)]
    pub content: HashMap<String, MediaType>,
    /// A map of operations links that can be followed from the response. The
    /// key of the map is a short name for the link, following the naming
    /// constraints of the names for [Component Objects].
    ///
    /// [Component Objects]: Components
    #[serde(default)]
    pub links: HashMap<String, Reference<Link>>,
}

/// Callback Object.
///
/// A map of possible out-of band callbacks related to the parent operation.
/// Each value in the map is a [Path Item Object] that describes a set of
/// requests that may be initiated by the API provider and the expected
/// responses. The key value used to identify the path item object is an
/// expression, evaluated at runtime, that identifies a URL to use for the
/// callback operation.
///
/// To describe incoming requests from the API provider independent from another
/// API call, use the [`webhooks`] field.
///
/// [Path Item Object]: PathItem
/// [`webhooks`]: Spec::webhooks
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Callback {
    /// A Path Item Object, or a reference to one, used to define a callback
    /// request and expected responses.
    #[serde(default)]
    pub expressions: HashMap<String, PathItem>, // NOTE: `PathItem` includes all fields of `Reference`.
}

/// Example Object.
///
/// In all cases, the example value is expected to be compatible with the type
/// schema of its associated value. Tooling implementations MAY choose to
/// validate compatibility automatically, and reject the example value(s) if
/// incompatible.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    /// Short description for the example.
    #[serde(default)]
    pub summary: Option<String>,
    /// Long description for the example. [CommonMark syntax] MAY be used for
    /// rich text representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// Embedded literal example. The `value` field and `externalValue` field
    /// are mutually exclusive. To represent examples of media types that cannot
    /// naturally represented in JSON or YAML, use a string value to contain the
    /// example, escaping where necessary.
    #[serde(default)]
    pub value: Option<Any>,
    /// A URI that points to the literal example. This provides the capability
    /// to reference examples that cannot easily be included in JSON or YAML
    /// documents. The `value` field and `externalValue` field are mutually
    /// exclusive.
    #[serde(default)]
    pub external_value: String,
}

/// The Link object represents a possible design-time link for a response.
///
/// The presence of a link does not guarantee the caller's ability to
/// successfully invoke it, rather it provides a known relationship and
/// traversal mechanism between responses and other operations.
///
/// Unlike _dynamic_ links (i.e. links provided **in** the response payload),
/// the OAS linking mechanism does not require link information in the runtime
/// response.
///
/// For computing links, and providing instructions to execute them, a [runtime
/// expression] is used for accessing values in an operation and using them as
/// parameters while invoking the linked operation.
///
/// A linked operation MUST be identified using either an `operationRef` or
/// `operationId`. In the case of an `operationId`, it MUST be unique and
/// resolved in the scope of the OAS document. Because of the potential for name
/// clashes, the `operationRef` syntax is preferred for OpenAPI documents with
/// external references.
///
/// [runtime expression]: RuntimeExpression
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    /// A relative or absolute URI reference to an OAS operation. This field is
    /// mutually exclusive of the `operationId` field, and MUST point to an
    /// [Operation Object]. Relative `operationRef` values MAY be used to locate
    /// an existing [Operation Object] in the OpenAPI definition.
    ///
    /// [Operation Object]: Operation
    #[serde(default)]
    pub operation_ref: Option<String>,
    /// The name of an _existing_, resolvable OAS operation, as defined with a
    /// unique `operationId`. This field is mutually exclusive of the
    /// `operationRef` field.
    #[serde(default)]
    pub operation_id: Option<String>,
    /// A map representing parameters to pass to an operation as specified with
    /// `operationId` or identified via `operationRef`. The key is the parameter
    /// name to be used, whereas the value can be a constant or an expression to
    /// be evaluated and passed to the linked operation. The parameter name can
    /// be qualified using the [parameter location] `[{in}.]{name}` for
    /// operations that use the same parameter name in different locations (e.g.
    /// path.id).
    ///
    /// [parameter location]: Parameter::in
    #[serde(default)]
    pub parameters: HashMap<String, RuntimeExpression>, // TODO: or `Any`.
    /// A literal value or [{expression}] to use as a request body when calling
    /// the target operation.
    ///
    /// [{expression}]: RuntimeExpression
    #[serde(default)]
    pub request_body: Option<RuntimeExpression>, // TODO: or `Any`.
    /// A description of the link. [CommonMark syntax] MAY be used for rich text
    /// representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// A server object to be used by the target operation.
    #[serde(default)]
    pub server: Option<Server>,
}

/// Runtime expressions allow defining values based on information that will
/// only be available within the HTTP message in an actual API call.
///
/// This mechanism is used by [Link Objects] and [Callback Objects].
///
/// The runtime expression is defined by the following [ABNF] syntax.
///
/// ```abnf
///       expression = ( "$url" / "$method" / "$statusCode" / "$request." source / "$response." source )
///       source = ( header-reference / query-reference / path-reference / body-reference )
///       header-reference = "header." token
///       query-reference = "query." name
///       path-reference = "path." name
///       body-reference = "body" ["#" json-pointer ]
///       json-pointer    = *( "/" reference-token )
///       reference-token = *( unescaped / escaped )
///       unescaped       = %x00-2E / %x30-7D / %x7F-10FFFF
///          ; %x2F ('/') and %x7E ('~') are excluded from 'unescaped'
///       escaped         = "~" ( "0" / "1" )
///         ; representing '~' and '/', respectively
///       name = *( CHAR )
///       token = 1*tchar
///       tchar = "!" / "#" / "$" / "%" / "&" / "'" / "*" / "+" / "-" / "." /
///         "^" / "_" / "`" / "|" / "~" / DIGIT / ALPHA
/// ```
///
/// Here, `json-pointer` is taken from [RFC6901], `char` from [RFC7159] and
/// `token` from [RFC7230]. The `name` identifier is case-sensitive, whereas
/// `token` is not.
///
/// [Link Objects]: Link
/// [Callback Objects]: Callback
/// [ABNF]: https://tools.ietf.org/html/rfc5234
/// [RFC6901]: https://tools.ietf.org/html/rfc6901
/// [RFC7159]: https://tools.ietf.org/html/rfc7159#section-7
/// [RFC7230]: https://tools.ietf.org/html/rfc7230#section-3.2.6
pub type RuntimeExpression = String;

/// Header Object.
///
/// The Header Object follows the structure of the [Parameter Object] with the
/// following changes:
///
/// 1. `name` MUST NOT be specified, it is given in the corresponding `headers`
///    map.
/// 2. `in` MUST NOT be specified, it is implicitly in `header`.
/// 3. All traits that are affected by the location MUST be applicable to a
///    location of `header` (for example, [`style`]).
///
/// [Parameter Object]: Parameter
/// [`style`]: Header::style
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    /// A brief description of the parameter. This could contain examples of
    /// use. [CommonMark syntax] MAY be used for rich text representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// Determines whether this parameter is mandatory. If the [parameter
    /// location] is `"path"`, this property is **REQUIRED** and its value MUST
    /// be `true`. Otherwise, the property MAY be included and its default value
    /// is `false`.
    ///
    /// [parameter location]: Parameter::in
    #[serde(default)]
    pub required: bool,
    /// Specifies that a parameter is deprecated and SHOULD be transitioned out
    /// of usage. Default value is `false`.
    #[serde(default)]
    pub deprecated: bool,
    /// Describes how the parameter value will be serialized depending on the
    /// type of the parameter value.
    ///
    /// Default value is `simple`.
    #[serde(default)]
    pub style: Option<HeaderStyle>,
    /// The schema defining the type used for the parameter.
    #[serde(default)]
    pub schema: Option<Schema>,
    /// Example of the parameter's potential value. The example SHOULD match the
    /// specified schema and encoding properties if present. The `example` field
    /// is mutually exclusive of the `examples` field. Furthermore, if
    /// referencing a `schema` that contains an example, the `example` value
    /// SHALL _override_ the example provided by the schema. To represent
    /// examples of media types that cannot naturally be represented in JSON or
    /// YAML, a string value can contain the example with escaping where
    /// necessary.
    #[serde(default)]
    pub example: Option<Any>,
    /// Examples of the parameter's potential value. Each example SHOULD contain
    /// a value in the correct format as specified in the parameter encoding.
    /// The `examples` field is mutually exclusive of the `example` field.
    /// Furthermore, if referencing a `schema` that contains an example, the
    /// `examples` value SHALL _override_ the example provided by the schema.
    #[serde(default)]
    pub examples: HashMap<String, Reference<Example>>,
    /// A map containing the representations for the parameter. The key is the
    /// media type and the value describes it. The map MUST only contain one
    /// entry.
    #[serde(default)]
    pub content: HashMap<String, MediaType>,
}

/// Header style.
///
/// See [`Header::style`].
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HeaderStyle {
    /// Simple style parameters defined by [RFC6570]. This option replaces
    /// `collectionFormat` with a `csv` value from OpenAPI 2.0.
    ///
    /// type: `array`
    /// in: `path`, `header`
    ///
    /// [RFC6570]: https://tools.ietf.org/html/rfc6570#section-3.2.2
    Simple,
}

/// Tag Object.
///
/// Adds metadata to a single tag that is used by the [Operation Object]. It is
/// not mandatory to have a Tag Object per tag defined in the Operation Object
/// instances.
///
/// [Operation Object]: Operation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// The name of the tag.
    pub name: String,
    /// A description for the tag. [CommonMark syntax] MAY be used for rich text
    /// representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(default)]
    pub external_docs: Option<ExternalDocument>,
}

/// A simple object to allow referencing other components in the OpenAPI
/// document, internally and externally.
///
/// The `$ref` string value contains a URI [RFC3986], which identifies the
/// location of the value being referenced.
///
/// [RFC3986]: https://tools.ietf.org/html/rfc3986
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference<T> {
    /// The reference identifier. This MUST be in the form of a URI.
    #[serde(rename = "$ref")]
    r#ref: Option<String>,
    /// A short summary which by default SHOULD override that of the
    /// referenced component. If the referenced object-type does not allow a
    /// `summary` field, then this field has no effect.
    #[serde(default)]
    summary: Option<String>,
    /// A description which by default SHOULD override that of the
    /// referenced component. [CommonMark syntax] MAY be used for rich text
    /// representation. If the referenced object-type does not allow a
    /// `description` field, then this field has no effect.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    description: Option<String>,
    /// Object `T`.
    #[serde(flatten)]
    object: Option<T>,
}

/// The Schema Object allows the definition of input and output data types.
///
/// These types can be objects, but also primitives and arrays. This object is a
/// superset of the [JSON Schema Specification Draft 2020-12].
///
/// For more information about the properties, see [JSON Schema Core] and [JSON
/// Schema Validation].
///
/// Unless stated otherwise, the property definitions follow those of JSON
/// Schema and do not add any additional semantics.
///
/// Where JSON Schema indicates that behavior is defined by the application
/// (e.g. for annotations), OAS also defers the definition of semantics to the
/// application consuming the OpenAPI document.
///
/// [JSON Schema Specification Draft 2020-12]: https://tools.ietf.org/html/draft-bhutton-json-schema-00
/// [JSON Schema Core]: https://tools.ietf.org/html/draft-bhutton-json-schema-00
/// [JSON Schema Validation]: https://tools.ietf.org/html/draft-bhutton-json-schema-validation-00
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    // JSON Schema Section 8. The JSON Schema Core Vocabulary
    /// The `$schema` keyword is both used as a JSON Schema dialect identifier
    /// and as the identifier of a resource which is itself a JSON Schema, which
    /// describes the set of valid schemas written for this particular dialect.
    ///
    /// The value of this keyword MUST be a [URI] (containing a scheme) and this
    /// URI MUST be normalized. The current schema MUST be valid against the
    /// meta-schema identified by this URI.
    ///
    /// If present this MUST be used to determine which dialect should be used
    /// when processing the schema. This allows use of Schema Objects which
    /// comply with other drafts of JSON Schema than the default Draft 2020-12
    /// support.
    ///
    /// [URI]: https://datatracker.ietf.org/doc/html/rfc3986
    #[serde(rename = "$schema", default)]
    pub schema: Option<String>,
    /// The `$id` keyword identifies a schema resource with its canonical [RFC
    /// 6596] URI.
    ///
    /// Note that this URI is an identifier and not necessarily a network
    /// locator. In the case of a network-addressable URL, a schema need not be
    /// downloadable from its canonical URI.
    ///
    /// [RFC 6596]: https://datatracker.ietf.org/doc/html/rfc6596
    #[serde(rename = "$id", default)]
    pub id: Option<String>,
    /// The `$ref` keyword is an applicator that is used to reference a
    /// statically identified schema. Its results are the results of the
    /// referenced schema.
    ///
    /// The value of the `$ref` keyword MUST be a string which is a URI-
    /// Reference. Resolved against the current URI base, it produces the URI of
    /// the schema to apply. This resolution is safe to perform on schema load,
    /// as the process of evaluating an instance cannot change how the reference
    /// resolves.
    #[serde(rename = "$ref", default)]
    pub r#ref: Option<String>,
    /// This keyword reserves a location for comments from schema authors to
    /// readers or maintainers of the schema.
    ///
    /// Implementations MUST NOT present this string to end users. Tools for
    /// editing schemas SHOULD support displaying and editing this keyword. The
    /// value of this keyword MAY be used in debug or error output which is
    /// intended for developers making use of schemas.
    #[serde(rename = "$comment", default)]
    pub comment: Option<String>,

    // JSON Schema Section 10.2.1. Keywords for Applying Subschemas With Logic
    /// An instance validates successfully against this keyword if it validates
    /// successfully against all schemas defined by this keyword's value.
    #[serde(default)]
    pub all_of: Option<Vec<Schema>>,
    /// An instance validates successfully against this keyword if it validates
    /// successfully against at least one schema defined by this keyword's
    /// value. Note that when annotations are being collected, all subschemas
    /// MUST be examined so that annotations are collected from each subschema
    /// that validates successfully.
    #[serde(default)]
    pub any_of: Option<Vec<Schema>>,
    /// An instance validates successfully against this keyword if it validates
    /// successfully against exactly one schema defined by this keyword's value.
    #[serde(default)]
    pub one_of: Option<Vec<Schema>>,
    /// An instance is valid against this keyword if it fails to validate
    /// successfully against the schema defined by this keyword.
    #[serde(default)]
    pub not: Option<Box<Schema>>,

    // JSON Schema Section 10.2.2. Keywords for Applying Subschemas
    // Conditionally
    /// This validation outcome of this keyword's subschema has no direct effect
    /// on the overall validation result. Rather, it controls which of the
    /// `then` or `else` keywords are evaluated.
    ///
    /// Instances that successfully validate against this keyword's subschema
    /// MUST also be valid against the subschema value of the `then` keyword, if
    /// present.
    ///
    /// Instances that fail to validate against this keyword's subschema MUST
    /// also be valid against the subschema value of the `else` keyword, if
    /// present.
    #[serde(default)]
    pub r#if: Option<Box<Schema>>,
    /// When `if` is present, and the instance successfully validates against
    /// its subschema, then validation succeeds against this keyword if the
    /// instance also successfully validates against this keyword's subschema.
    ///
    /// This keyword has no effect when `if` is absent, or when the instance
    /// fails to validate against its subschema. Implementations MUST NOT
    /// evaluate the instance against this keyword, for either validation or
    /// annotation collection purposes, in such cases.
    #[serde(default)]
    pub then: Option<Box<Schema>>,
    /// When `if` is present, and the instance fails to validate against its
    /// subschema, then validation succeeds against this keyword if the instance
    /// successfully validates against this keyword's subschema.
    ///
    /// This keyword has no effect when `if` is absent, or when the instance
    /// successfully validates against its subschema. Implementations MUST NOT
    /// evaluate the instance against this keyword, for either validation or
    /// annotation collection purposes, in such cases.
    #[serde(default)]
    pub r#else: Option<Box<Schema>>,
    /// This keyword specifies subschemas that are evaluated if the instance is
    /// an object and contains a certain property.
    ///
    /// If the object key is a property in the instance, the entire instance
    /// must validate against the subschema. Its use is dependent on the
    /// presence of the property.
    ///
    /// Omitting this keyword has the same behavior as an empty object.
    #[serde(default)]
    pub dependent_schemas: HashMap<String, Schema>,

    // JSON Schema Section 10.3.1. Keywords for Applying Subschemas to Arrays
    /// Validation succeeds if each element of the instance validates against
    /// the schema at the same position, if any. This keyword does not constrain
    /// the length of the array. If the array is longer than this keyword's
    /// value, this keyword validates only the prefix of matching length.
    ///
    /// This keyword produces an annotation value which is the largest index to
    /// which this keyword applied a subschema. The value MAY be a boolean true
    /// if a subschema was applied to every index of the instance, such as is
    /// produced by the `items` keyword. This annotation affects the behavior of
    /// `items` and `unevaluated_items`.
    ///
    /// Omitting this keyword has the same assertion behavior as an empty array.
    #[serde(default)]
    pub prefix_items: Vec<Schema>,
    /// This keyword applies its subschema to all instance elements at indexes
    /// greater than the length of the `prefix_iItems` array in the same schema
    /// object, as reported by the annotation result of that `prefix_items`
    /// keyword. If no such annotation result exists, `items` applies its
    /// subschema to all instance array elements.
    ///
    /// If the `items` subschema is applied to any positions within the instance
    /// array, it produces an annotation result of boolean true, indicating that
    /// all remaining array elements have been evaluated against this keyword's
    /// subschema.
    ///
    /// Omitting this keyword has the same assertion behavior as an empty
    /// schema.
    ///
    /// Implementations MAY choose to implement or optimize this keyword in
    /// another way that produces the same effect, such as by directly checking
    /// for the presence and size of a `prefix_items` array. Implementations
    /// that do not support annotation collection MUST do so.
    #[serde(default)]
    pub items: Option<Box<Schema>>,
    /// An array instance is valid against `contains` if at least one of its
    /// elements is valid against the given schema. The subschema MUST be
    /// applied to every array element even after the first match has been
    /// found, in order to collect annotations for use by other keywords. This
    /// is to ensure that all possible annotations are collected. Logically, the
    /// validation result of applying the value subschema to each item in the
    /// array MUST be ORed with `false`, resulting in an overall validation
    /// result.
    ///
    /// This keyword produces an annotation value which is an array of the
    /// indexes to which this keyword validates successfully when applying its
    /// subschema, in ascending order. The value MAY be a boolean `true` if the
    /// subschema validates successfully when applied to every index of the
    /// instance. The annotation MUST be present if the instance array to which
    /// this keyword's schema applies is empty.
    #[serde(default)]
    pub contains: Option<Box<Schema>>,

    // JSON Schema Section 10.3.2. Keywords for Applying Subschemas to Objects
    /// Validation succeeds if, for each name that appears in both the instance
    /// and as a name within this keyword's value, the child instance for that
    /// name successfully validates against the corresponding schema.
    ///
    /// The annotation result of this keyword is the set of instance property
    /// names matched by this keyword.
    ///
    /// Omitting this keyword has the same assertion behavior as an empty
    /// object.
    #[serde(default)]
    pub properties: Option<HashMap<String, Schema>>,
    /// Each property name of this object SHOULD be a valid regular expression,
    /// according to the ECMA-262 regular expression dialect. Each property
    /// value of this object MUST be a valid JSON Schema.
    ///
    /// Validation succeeds if, for each instance name that matches any regular
    /// expressions that appear as a property name in this keyword's value, the
    /// child instance for that name successfully validates against each schema
    /// that corresponds to a matching regular expression.
    ///
    /// The annotation result of this keyword is the set of instance property
    /// names matched by this keyword.
    ///
    /// Omitting this keyword has the same assertion behavior as an empty
    /// object.
    #[serde(default)]
    pub pattern_properties: HashMap<String, Schema>,
    /// The behavior of this keyword depends on the presence and annotation
    /// results of `properties` and `pattern_properties` within the same schema
    /// object. Validation with `additional_properties` applies only to the
    /// child values of instance names that do not appear in the annotation
    /// results of either `properties` or `pattern_properties`.
    ///
    /// For all such properties, validation succeeds if the child instance
    /// validates against the `additional_properties` schema.
    ///
    /// The annotation result of this keyword is the set of instance property
    /// names validated by this keyword's subschema.
    ///
    /// Omitting this keyword has the same assertion behavior as an empty
    /// schema.
    ///
    /// Implementations MAY choose to implement or optimize this keyword in
    /// another way that produces the same effect, such as by directly checking
    /// the names in `properties` and the patterns in `pattern_properties`
    /// against the instance property set. Implementations that do not support
    /// annotation collection MUST do so.
    #[serde(default)]
    pub additional_properties: Option<Box<Schema>>,
    /// If the instance is an object, this keyword validates if every property
    /// name in the instance validates against the provided schema. Note the
    /// property name that the schema is testing will always be a string.
    ///
    /// Omitting this keyword has the same behavior as an empty schema.
    #[serde(default)]
    pub property_names: Option<Box<Schema>>,

    // JSON Schema Section 11. A Vocabulary for Unevaluated Locations
    /// The behavior of this keyword depends on the annotation results of
    /// adjacent keywords that apply to the instance location being validated.
    /// Specifically, the annotations from `prefix_items`, `items`, and
    /// `contains`, which can come from those keywords when they are adjacent to
    /// the `unevaluated_items` keyword. Those three annotations, as well as
    /// `unevaluated_items`, can also result from any and all adjacent in-place
    /// applicator (Section 10.2) keywords. This includes but is not limited to
    /// the in-place applicators defined in this document.
    ///
    /// If no relevant annotations are present, the `unevaluated_items`
    /// subschema MUST be applied to all locations in the array. If a boolean
    /// true value is present from any of the relevant annotations,
    /// `unevaluated_items` MUST be ignored. Otherwise, the subschema MUST be
    /// applied to any index greater than the largest annotation value for
    /// `prefix_items`, which does not appear in any annotation value for
    /// `contains`.
    ///
    /// This means that `prefix_items`, `items`, `contains`, and all in-place
    /// applicators MUST be evaluated before this keyword can be evaluated.
    /// Authors of extension keywords MUST NOT define an in-place applicator
    /// that would need to be evaluated after this keyword.
    ///
    /// If the `unevaluatedItems` subschema is applied to any positions within
    /// the instance array, it produces an annotation result of boolean true,
    /// analogous to the behavior of `items`.
    ///
    /// Omitting this keyword has the same assertion behavior as an empty
    /// schema.
    #[serde(default)]
    pub unevaluated_items: Option<Box<Schema>>,
    /// The behavior of this keyword depends on the annotation results of
    /// adjacent keywords that apply to the instance location being validated.
    /// Specifically, the annotations from "properties", "patternProperties",
    /// and `additional_properties`, which can come from those keywords when
    /// they are adjacent to the `unevaluated_properties` keyword. Those three
    /// annotations, as well as `unevaluated_properties`, can also result from
    /// any and all adjacent in-place applicator keywords. This includes but is
    /// not limited to the in-place applicators defined in this document.
    ///
    /// Validation with `unevaluated_properties` applies only to the child
    /// values of instance names that do not appear in the `properties`,
    /// `pattern_properties`, `additional_properties`, or
    /// `unevaluated_properties` annotation results that apply to the instance
    /// location being validated.
    ///
    /// For all such properties, validation succeeds if the child instance
    /// validates against the `unevaluated_properties` schema.
    ///
    /// This means that `properties`, `pattern_properties`,
    /// `additional_properties`, and all in-place applicators MUST be evaluated
    /// before this keyword can be evaluated. Authors of extension keywords MUST
    /// NOT define an in-place applicator that would need to be evaluated after
    /// this keyword.
    ///
    /// The annotation result of this keyword is the set of instance property
    /// names validated by this keyword's subschema.
    ///
    /// Omitting this keyword has the same assertion behavior as an empty
    /// schema.
    #[serde(default)]
    pub unevaluated_properties: Option<Box<Schema>>,

    // JSON Schema Validation Section 6.1. Validation Keywords for Any Instance
    // Type
    /// Data type.
    #[serde(with = "one_or_array", default)]
    pub r#type: Vec<Type>,
    /// Valid values for this schema.
    #[serde(default)]
    pub r#enum: Vec<String>,
    /// Use of this keyword is functionally equivalent to an [`enum`] with a
    /// single value.
    ///
    /// [`enum`]: Schema::enum
    #[serde(default)]
    pub r#const: Option<String>,

    // JSON Schema Validation Section 6.2. Validation Keywords for Numeric
    // Instances (number and integer)
    /// A numeric instance is valid only if division by this keyword's value
    /// results in an integer.
    #[serde(default)]
    pub multiple_of: Option<f64>,
    /// If the instance is a number, then this keyword validates only if the
    /// instance is less than or exactly equal to `maximum`.
    #[serde(default)]
    pub maximum: Option<f64>,
    /// If the instance is a number, then the instance is valid only if it has a
    /// value strictly less than (not equal to) `exclusiveMaximum`.
    #[serde(default)]
    pub exclusive_maximum: Option<f64>,
    /// If the instance is a number, then this keyword validates only if the
    /// instance is greater than or exactly equal to `minimum`.
    #[serde(default)]
    pub minimum: Option<f64>,
    /// If the instance is a number, then the instance is valid only if it has a
    /// value strictly greater than (not equal to) `exclusiveMinimum`.
    #[serde(default)]
    pub exclusive_minimum: Option<f64>,

    // JSON Schema Validation Section 6.3. Validation Keywords for Strings
    /// A string instance is valid against this keyword if its length is less
    /// than, or equal to, the value of this keyword.
    #[serde(default)]
    pub max_length: Option<usize>,
    /// A string instance is valid against this keyword if its length is greater
    /// than, or equal to, the value of this keyword.
    #[serde(default)]
    pub min_length: Option<usize>,
    /// A string instance is considered valid if the regular expression matches
    /// the instance successfully. Recall: regular expressions are not
    /// implicitly anchored.
    #[serde(default)]
    pub pattern: Option<String>,

    // JSON Schema Validation Section 6.4. Validation Keywords for Arrays
    /// An array instance is valid against `maxItems` if its size is less than,
    /// or equal to, the value of this keyword.
    #[serde(default)]
    pub max_items: Option<usize>,
    /// An array instance is valid against `minItems` if its size is greater
    /// than, or equal to, the value of this keyword.
    #[serde(default)]
    pub min_items: Option<usize>,
    /// If this keyword has boolean value false, the instance validates
    /// successfully. If it has boolean value true, the instance validates
    /// successfully if all of its elements are unique.
    #[serde(default)]
    pub unique_ttems: bool,
    /// If `contains` is not present within the same schema object, then this
    /// keyword has no effect.
    ///
    /// An instance array is valid against `maxContains` in two ways, depending
    /// on the form of the annotation result of an adjacent `contains`
    /// json-schema keyword. The first way is if the annotation result is an
    /// array and the length of that array is less than or equal to the
    /// `maxContains` value. The second way is if the annotation result is a
    /// boolean `true` and the instance array length is less than or equal to
    /// the `maxContains` value.
    #[serde(default)]
    pub max_contains: Option<usize>,
    /// If `contains` is not present within the same schema object, then this
    /// keyword has no effect.
    ///
    /// An instance array is valid against `minContains` in two ways, depending
    /// on the form of the annotation result of an adjacent `contains`
    /// [json-schema] keyword. The first way is if the annotation result is an
    /// array and the length of that array is greater than or equal to the
    /// `minContains` value. The second way is if the annotation result is a
    /// boolean `true` and the instance array length is greater than or equal to
    /// the `minContains` value.
    ///
    /// A value of 0 is allowed, but is only useful for setting a range of
    /// occurrences from 0 to the value of `maxContains`. A value of 0 with no
    /// `maxContains` causes `contains` to always pass validation.
    ///
    /// Omitting this keyword has the same behavior as a value of 1.
    #[serde(default)]
    pub min_contains: Option<usize>,

    // JSON Schema Validation Section 6.5. Validation Keywords for Objects
    /// An object instance is valid against `maxProperties` if its number of
    /// properties is less than, or equal to, the value of this keyword.
    #[serde(default)]
    pub max_properties: Option<usize>,
    /// An object instance is valid against `minProperties` if its number of
    /// properties is greater than, or equal to, the value of this keyword.
    ///
    /// Omitting this keyword has the same behavior as a value of 0.
    #[serde(default)]
    pub min_properties: Option<usize>,
    /// An object instance is valid against this keyword if every item in the
    /// array is the name of a property in the instance.
    ///
    /// Omitting this keyword has the same behavior as an empty array.
    #[serde(default)]
    pub required: Vec<String>,
    /// This keyword specifies properties that are required if a specific other
    /// property is present. Their requirement is dependent on the presence of
    /// the other property.
    ///
    /// Validation succeeds if, for each name that appears in both the instance
    /// and as a name within this keyword's value, every item in the
    /// corresponding array is also the name of a property in the instance.
    ///
    /// Omitting this keyword has the same behavior as an empty object.
    #[serde(default)]
    pub dependent_required: HashMap<String, Vec<String>>,

    // JSON Schema Validation Section 7. Vocabularies for Semantic Content With
    // "format"
    /// The `format` annotation keyword is defined to allow schema authors to
    /// convey semantic information for a fixed subset of values which are
    /// accurately described by authoritative resources, be they RFCs or other
    /// external specifications.
    #[serde(default)]
    pub format: Option<FormatOrString>,

    // JSON Schema Validation Section 8. A Vocabulary for the Contents of
    // String-Encoded Data
    /// If the instance value is a string, this property defines that the string
    /// SHOULD be interpreted as binary data and decoded using the encoding
    /// named by this property.
    ///
    /// Possible values indicating base 16, 32, and 64 encodings with several
    /// variations are listed in [RFC 4648]. Additionally, sections 6.7 and 6.8
    /// of [RFC 2045] provide encodings used in MIME. As `base64` is defined in
    /// both RFCs, the definition from RFC 4648 SHOULD be assumed unless the
    /// string is specifically intended for use in a MIME context. Note that all
    /// of these encodings result in strings consisting only of 7-bit ASCII
    /// characters. Therefore, this keyword has no meaning for strings
    /// containing characters outside of that range.
    ///
    /// If this keyword is absent, but `content_media_type` is present, this
    /// indicates that the encoding is the identity encoding, meaning that no
    /// transformation was needed in order to represent the content in a UTF-8
    /// string.
    ///
    /// [RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648
    /// [RFC 2045]: https://datatracker.ietf.org/doc/html/rfc2045
    #[serde(default)]
    pub content_encoding: Option<String>,
    /// If the instance is a string, this property indicates the media type of
    /// the contents of the string. If `content_encoding` is present, this
    /// property describes the decoded string.
    ///
    /// The value of this property MUST be a string, which MUST be a media type,
    /// as defined by [RFC 2046].
    ///
    /// [RFC 2046]: https://datatracker.ietf.org/doc/html/rfc2046
    #[serde(default)]
    pub content_media_type: Option<String>,
    /// If the instance is a string, and if `content_media_type` is present,
    /// this property contains a schema which describes the structure of the
    /// string.
    ///
    /// This keyword MAY be used with any media type that can be mapped into
    /// JSON Schema's data model.
    ///
    /// The value of this property MUST be a valid JSON schema. It SHOULD be
    /// ignored if `content_media_type` is not present.
    #[serde(default)]
    pub content_schema: Option<Box<Schema>>,

    // JSON Schema Validation Section 9. A Vocabulary for Basic Meta-Data
    // Annotations
    /// A title will preferably be short explanation about the purpose of the
    /// instance described by this schema.
    #[serde(default)]
    pub title: Option<String>,
    /// A description will provide explanation about the purpose of the instance
    /// described by this schema. [CommonMark syntax] MAY be used for rich text
    /// representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// This keyword can be used to supply a default JSON value associated with
    /// a particular schema. It is RECOMMENDED that a default value be valid
    /// against the associated schema.
    #[serde(default)]
    pub default: Option<Any>,
    /// If `deprecated` has a value of boolean true, it indicates that
    /// applications SHOULD refrain from usage of the declared property. It MAY
    /// mean the property is going to be removed in the future.
    ///
    /// A root schema containing `deprecated` with a value of true indicates
    /// that the entire resource being described MAY be removed in the future.
    ///
    /// The `deprecated` keyword applies to each instance location to which the
    /// schema object containing the keyword successfully applies. This can
    /// result in scenarios where every array item or object property is
    /// deprecated even though the containing array or object is not.
    ///
    /// Omitting this keyword has the same behavior as a value of false.
    #[serde(default)]
    pub deprecated: bool,
    /// If `read_only` has a value of boolean true, it indicates that the value
    /// of the instance is managed exclusively by the owning authority, and
    /// attempts by an application to modify the value of this property are
    /// expected to be ignored or rejected by that owning authority.
    ///
    /// An instance document that is marked as `read_only` for the entire
    /// document MAY be ignored if sent to the owning authority, or MAY result
    /// in an error, at the authority's discretion.
    ///
    /// Omitting this keyword has the same behavior as values of false.
    #[serde(default)]
    pub read_only: bool,
    /// If `write_only` has a value of boolean true, it indicates that the value
    /// is never present when the instance is retrieved from the owning
    /// authority. It can be present when sent to the owning authority to update
    /// or create the document (or the resource it represents), but it will not
    /// be included in any updated or newly created version of the instance.
    ///
    /// An instance document that is marked as `writeOnly` for the entire
    /// document MAY be returned as a blank document of some sort, or MAY
    /// produce an error upon retrieval, or have the retrieval request ignored,
    /// at the authority's discretion.
    ///
    /// Omitting this keyword has the same behavior as values of false.
    #[serde(default)]
    pub write_only: bool,
    /// This keyword can be used to provide sample JSON values associated with a
    /// particular schema, for the purpose of illustrating usage. It is
    /// RECOMMENDED that these values be valid against the associated schema.
    ///
    /// Implementations MAY use the value(s) of `default`, if present, as an
    /// additional example. If `examples` is absent, `default` MAY still be used
    /// in this manner.
    #[serde(default)]
    pub examples: Vec<Any>,

    // OpenAPI spec extension.
    /// Adds support for polymorphism. The discriminator is an object name that
    /// is used to differentiate between other schemas which may satisfy the
    /// payload description.
    #[serde(default)]
    pub discriminator: Option<Discriminator>,
    /// This MAY be used only on properties schemas. It has no effect on root
    /// schemas. Adds additional metadata to describe the XML representation of
    /// this property.
    #[serde(default)]
    pub xml: Option<Xml>,
    /// Additional external documentation for this schema.
    #[serde(default)]
    pub external_docs: Option<ExternalDocument>,
    /// A free-form property to include an example of an instance for this
    /// schema. To represent examples that cannot be naturally represented in
    /// JSON or YAML, a string value can be used to contain the example with
    /// escaping where necessary.
    ///
    /// **Deprecated:** The `example` property has been deprecated in favor of
    /// the JSON Schema `examples` keyword. Use of `example` is discouraged, and
    /// later versions of this specification may remove it.
    #[serde(default)]
    pub example: Option<Any>,
    /// Allows the schema to be extended. The value can be `null`/`None`, a
    /// primitive, an array or an object.
    #[serde(flatten)]
    pub extensions: HashMap<String, Any>,
}

mod one_or_array {
    //! Deserialize and Serialize functions for [`Schema::type`].
    //!
    //! Accepts either 1 type (string) or an array of types (strings).
    //!
    //! [`Schema::type`]: crate::Schema::type

    use std::fmt;

    use serde::de::{Error, IntoDeserializer, SeqAccess, Visitor};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::Type;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Type>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OneOrArray;

        impl<'de> Visitor<'de> for OneOrArray {
            type Value = Vec<Type>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("one or more types")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let r#type = Type::deserialize(v.into_deserializer())?;
                let mut types = Vec::with_capacity(1);
                types.push(r#type);
                Ok(types)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut types = Vec::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(r#type) = seq.next_element()? {
                    types.push(r#type);
                }
                Ok(types)
            }
        }

        deserializer.deserialize_any(OneOrArray)
    }

    pub(crate) fn serialize<S>(r#type: &Vec<Type>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match r#type.len() {
            1 => r#type[0].serialize(serializer),
            _ => r#type.serialize(serializer),
        }
    }
}

/// Data type defined by [JSON Schema Validation Section 6.1.1].
///
/// [JSON Schema Validation Section 6.1.1]: https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-00#section-6.1.1
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}

/// Either a known [`Format`] or falls back to a string.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormatOrString {
    Format(Format),
    Other(String),
}

/// Data format defined by [JSON Schema Validation Section 7.3] and extended by
/// the OpenAPI spec.
///
/// [JSON Schema Validation Section 7.3]: https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-00#section-7.3
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Format {
    // JSON Schema Validation Section 7.3.1. Dates, Times, and Duration
    /// A string instance is valid against this attribute if it is a valid
    /// representation according to the "date-time" production.
    #[serde(rename = "date-time")]
    DateTime,
    /// A string instance is valid against this attribute if it is a valid
    /// representation according to the "full-date" production.
    #[serde(alias = "full-date")]
    Date,
    /// A string instance is valid against this attribute if it is a valid
    /// representation according to the "full-time" production.
    #[serde(alias = "full-time")]
    #[serde(alias = "partial-time")] // TODO: not sure if this valid.
    Time,
    /// A string instance is valid against this attribute if it is a valid
    /// representation according to the "duration" production.
    Duration,

    // JSON Schema Validation Section 7.3.2. Email Addresses
    /// As defined by the "Mailbox" ABNF rule in [RFC 5321, section 4.1.2].
    ///
    /// [RFC 5321, section 4.1.2]: https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.2
    Email,
    /// As defined by the extended "Mailbox" ABNF rule in [RFC 6531, section
    /// 3.3].
    ///
    /// [RFC 6531, section 3.3]: https://datatracker.ietf.org/doc/html/rfc6531#section-3.3
    #[serde(rename = "idn-email")]
    IdnEmail,

    // JSON Schema Validation Section 7.3.3. Hostnames
    /// As defined by [RFC 1123, section 2.1], including host names produced
    /// using the Punycode algorithm specified in [RFC 5891, section 4.4].
    ///
    /// [RFC 1123, section 2.1]: https://datatracker.ietf.org/doc/html/rfc1123#section-2.1
    /// [RFC 5891, section 4.4]: https://datatracker.ietf.org/doc/html/rfc5891#section-4.4
    Hostname,
    /// As defined by either [RFC 1123] as for hostname, or an internationalized
    /// hostname as defined by [RFC 5890, section 2.3.2.3].
    ///
    /// [RFC 1123]: https://datatracker.ietf.org/doc/html/rfc1123#section-2.1
    /// [RFC 5890]: https://datatracker.ietf.org/doc/html/rfc5890#section-2.3.2.3
    #[serde(rename = "idn-hostname")]
    IdnHostname,

    // JSON Schema Validation Section 7.3.4. IP Addresses
    /// An IPv4 address according to the "dotted-quad" ABNF syntax as defined in
    /// [RFC 2673, section 3.2].
    ///
    /// [RFC 2673, section 3.2]: https://datatracker.ietf.org/doc/html/rfc2673#section-3.2
    Ipv4,
    /// An IPv6 address as defined in [RFC 4291, section 2.2].
    ///
    /// [RFC 4291, section 2.2]: https://datatracker.ietf.org/doc/html/rfc4291#section-2.2
    Ipv6,

    // JSON Schema Validation Section 7.3.5. Resource Identifiers
    /// A string instance is valid against this attribute if it is a valid URI,
    /// according to [RFC3986].
    ///
    /// [RFC3986]: https://datatracker.ietf.org/doc/html/rfc3986
    #[serde(alias = "url")] // Commonly seen.
    Uri,
    /// A string instance is valid against this attribute if it is a valid URI
    /// Reference (either a URI or a relative- reference), according to
    /// [RFC3986].
    ///
    /// [RFC3986]: https://datatracker.ietf.org/doc/html/rfc3986
    #[serde(rename = "uri-reference")]
    UriReference,
    /// A string instance is valid against this attribute if it is a valid IRI,
    /// according to [RFC3987].
    ///
    /// [RFC3987]: https://datatracker.ietf.org/doc/html/rfc3987
    Iri,
    /// A string instance is valid against this attribute if it is a valid IRI
    /// Reference (either an IRI or a relative- reference), according to
    /// [RFC3987].
    ///
    /// [RFC3987]: https://datatracker.ietf.org/doc/html/rfc3987
    #[serde(rename = "iri-reference")]
    IriReference,
    /// A string instance is valid against this attribute if it is a valid
    /// string representation of a UUID, according to [RFC4122].
    ///
    /// Note that the "uuid" format is for plain UUIDs, not UUIDs in URNs. An
    /// example is "f81d4fae-7dec-11d0-a765-00a0c91e6bf6". For UUIDs as URNs,
    /// use the "uri" format, with a "pattern" regular expression of
    /// "^urn:uuid:" to indicate the URI scheme and URN namespace.
    ///
    /// [RFC4122]: https://datatracker.ietf.org/doc/html/rfc4122
    Uuid,

    // JSON Schema Validation Section 7.3.6. uri-template
    /// A string instance is valid against this attribute if it is a valid URI
    /// Template (of any level), according to [RFC6570].
    ///
    /// Note that URI Templates may be used for IRIs; there is no separate IRI
    /// Template specification.
    ///
    /// [RFC6570]: https://datatracker.ietf.org/doc/html/rfc6570
    #[serde(rename = "uri-template")]
    UriTemplate,

    // JSON Schema Validation Section 7.3.7. JSON Pointers
    /// A string instance is valid against this attribute if it is a valid JSON
    /// string representation of a JSON Pointer, according to [RFC 6901, section
    /// 5].
    ///
    /// [RFC 6901, section 5]: https://datatracker.ietf.org/doc/html/rfc6901#section-5
    #[serde(rename = "json-pointer")]
    JsonPointer,
    /// A string instance is valid against this attribute if it is a valid
    /// Relative JSON Pointer.
    #[serde(rename = "relative-json-pointer")]
    RelativeJsonPointer,

    // JSON Schema Validation Section 7.3.8. regex
    /// A regular expression, which SHOULD be valid according to the [ECMA-262]
    /// regular expression dialect.
    ///
    /// Implementations that validate formats MUST accept at least the subset of
    /// ECMA-262 defined in the Regular Expressions (Section 4.3) section of
    /// this specification, and SHOULD accept all valid ECMA-262 expressions.
    ///
    /// [ECMA-262]: https://www.ecma-international.org/ecma-262/11.0
    Regex,

    // Commonly seen.
    /// Binary data.
    Binary,
    /// An IPv4 or IPv6 address, also see the `Ipv4` and `Ipv6` variants.
    Ip,

    // OpenAPI spec extension.
    /// Signed 32 bits integer.
    Int32,
    /// Signed 64 bits integer (a.k.a. long).
    Int64,
    /// 32 bit floating point number.
    Float,
    /// 64 bit floating point number.
    Double,
    /// A hint to UIs to obscure input.
    Password,
}

/// Discriminator Object.
///
/// When request bodies or response payloads may be one of a number of different
/// schemas, a `discriminator` object can be used to aid in serialization,
/// deserialization, and validation. The discriminator is a specific object in a
/// schema which is used to inform the consumer of the document of an
/// alternative schema based on the value associated with it.
///
/// When using the discriminator, _inline_ schemas will not be considered.
///
/// The discriminator object is legal only when using one of the composite
/// keywords `oneOf`, `anyOf`, `allOf`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discriminator {
    /// The name of the property in the payload that will hold the discriminator
    /// value.
    pub property_name: String,
    /// An object to hold mappings between payload values and schema names or
    /// references.
    #[serde(default)]
    pub mapping: HashMap<String, String>,
}

/// XML Object.
///
/// A metadata object that allows for more fine-tuned XML model definitions.
///
/// When using arrays, XML element names are *not* inferred (for singular/plural
/// forms) and the `name` property SHOULD be used to add that information.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Xml {
    /// Replaces the name of the element/attribute used for the described schema
    /// property. When defined within `items`, it will affect the name of the
    /// individual XML elements within the list. When defined alongside `type`
    /// being `array` (outside the `items`), it will affect the wrapping element
    /// and only if `wrapped` is `true`. If `wrapped` is `false`, it will be
    /// ignored.
    #[serde(default)]
    pub name: Option<String>,
    /// The URI of the namespace definition. This MUST be in the form of an
    /// absolute URI.
    #[serde(default)]
    pub namespace: Option<String>,
    /// The prefix to be used for the [`name`].
    ///
    /// [`name`]: Xml::name
    #[serde(default)]
    pub prefix: Option<String>,
    /// Declares whether the property definition translates to an attribute
    /// instead of an element. Default value is `false`.
    #[serde(default)]
    pub attribute: bool,
    /// MAY be used only for an array definition. Signifies whether the array is
    /// wrapped (for example, `<books><book/><book/></books>`) or unwrapped
    /// (`<book/><book/>`). Default value is `false`. The definition takes
    /// effect only when defined alongside `type` being `array` (outside the
    /// `items`).
    #[serde(default)]
    pub wrapped: bool,
}

/// Security Scheme Object.
///
/// Defines a security scheme that can be used by the operations.
///
/// Supported schemes are HTTP authentication, an API key (either as a header, a
/// cookie parameter or as a query parameter), mutual TLS (use of a client
/// certificate), OAuth2's common flows (implicit, password, client credentials
/// and authorization code) as defined in [RFC6749], and [OpenID Connect
/// Discovery]. Please note that as of 2020, the implicit flow is about to be
/// deprecated by [OAuth 2.0 Security Best Current Practice]. Recommended for
/// most use case is Authorization Code Grant flow with PKCE.
///
/// [RFC6749]: https://tools.ietf.org/html/rfc6749
/// [OpenID Connect Discovery]: https://tools.ietf.org/html/draft-ietf-oauth-discovery-06
/// [OAuth 2.0 Security Best Current Practice]: https://tools.ietf.org/html/draft-ietf-oauth-security-topics
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityScheme {
    /// The type of the security scheme.
    pub r#type: SecuritySchemeType,
    /// A description for security scheme. [CommonMark syntax] MAY be used for
    /// rich text representation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    #[serde(default)]
    pub description: Option<String>,
    /// The name of the header, query or cookie parameter to be used.
    ///
    /// Required for [`SecuritySchemeType::ApiKey`].
    #[serde(default)]
    pub name: Option<String>,
    /// The location of the API key.
    ///
    /// Required for [`SecuritySchemeType::ApiKey`].
    #[serde(default)]
    pub r#in: Option<SecuritySchemeIn>,
    /// The name of the HTTP Authorization scheme to be used in the
    /// [Authorization header as defined in RFC7235]. The values used SHOULD be
    /// registered in the [IANA Authentication Scheme registry].
    ///
    /// Required for [`SecuritySchemeType::Http`].
    ///
    /// [Authorization header as defined in RFC7235]: https://tools.ietf.org/html/rfc7235#section-5.1
    /// [IANA Authentication Scheme registry]: https://www.iana.org/assignments/http-authschemes/http-authschemes.xhtml
    #[serde(default)]
    pub scheme: Option<String>,
    /// A hint to the client to identify how the bearer token is formatted.
    /// Bearer tokens are usually generated by an authorization server, so this
    /// information is primarily for documentation purposes.
    ///
    /// Required for [`SecuritySchemeType::Http`] (`"bearer"`).
    #[serde(default)]
    pub bearer_format: Option<String>,
    /// An object containing configuration information for the flow types
    /// supported.
    ///
    /// Required for [`SecuritySchemeType::Oauth2`].
    #[serde(default)]
    pub flows: Option<OauthFlows>,
    /// OpenId Connect URL to discover OAuth2 configuration values. This MUST be
    /// in the form of a URL. The OpenID Connect standard requires the use of
    /// TLS.
    ///
    /// Required for [`SecuritySchemeType::OpenIdConnect`].
    #[serde(default)]
    pub open_id_connect_url: Option<String>,
}

/// [`SecurityScheme::type`].
#[derive(Debug, Serialize, Deserialize)]
pub enum SecuritySchemeType {
    #[serde(rename = "apiKey")]
    ApiKey,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "mutualTLS")]
    MutualTls,
    #[serde(rename = "oauth2")]
    Oauth2,
    #[serde(rename = "openIdConnect")]
    OpenIdConnect,
}

/// [`SecurityScheme::in`].
#[derive(Debug, Serialize, Deserialize)]
pub enum SecuritySchemeIn {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "cookie")]
    Cookie,
}

/// Allows configuration of the supported OAuth Flows.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OauthFlows {
    /// Configuration for the OAuth Implicit flow
    #[serde(default)]
    pub implicit: Option<OauthFlow>,
    /// Configuration for the OAuth Resource Owner Password flow
    #[serde(default)]
    pub password: Option<OauthFlow>,
    /// Configuration for the OAuth Client Credentials flow. Previously called
    /// `application` in OpenAPI 2.0.
    #[serde(default)]
    pub client_credentials: Option<OauthFlow>,
    /// Configuration for the OAuth Authorization Code flow. Previously called
    /// `accessCode` in OpenAPI 2.0.
    #[serde(default)]
    pub authorization_code: Option<OauthFlow>,
}

/// Configuration details for a supported OAuth Flow.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OauthFlow {
    /// The authorization URL to be used for this flow. This MUST be in the form
    /// of a URL. The OAuth2 standard requires the use of TLS.
    ///
    /// Applies to `oauth2` (`"implicit"`, `"authorizationCode"`).
    pub authorization_url: String,
    /// The token URL to be used for this flow. This MUST be in the form of a
    /// URL. The OAuth2 standard requires the use of TLS.
    ///
    /// Applies to `oauth2` (`"password"`, `"clientCredentials"`,
    /// `"authorizationCode"`).
    pub token_url: String,
    /// The URL to be used for obtaining refresh tokens. This MUST be in the
    /// form of a URL. The OAuth2 standard requires the use of TLS.
    #[serde(default)]
    pub refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme. A map between the
    /// scope name and a short description for it. The map MAY be empty.
    pub scopes: HashMap<String, String>,
}

/// Security Requirement Object.
///
/// Lists the required security schemes to execute this operation. The name used
/// for each property MUST correspond to a security scheme declared in the
/// [Security Schemes] under the [Components Object].
///
/// Security Requirement Objects that contain multiple schemes require that all
/// schemes MUST be satisfied for a request to be authorized. This enables
/// support for scenarios where multiple query parameters or HTTP headers are
/// required to convey security information.
///
/// When a list of Security Requirement Objects is defined on the [OpenAPI
/// Object] or [Operation Object], only one of the Security Requirement Objects
/// in the list needs to be satisfied to authorize the request.
///
/// Each name MUST correspond to a security scheme which is declared in the
/// [Security Schemes] under the [Components Object]. If the security scheme is
/// of type `"oauth2"` or `"openIdConnect"`, then the value is a list of scope
/// names required for the execution, and the list MAY be empty if authorization
/// does not require a specified scope. For other security scheme types, the
/// array MAY contain a list of role names which are required for the execution,
/// but are not otherwise defined or exchanged in-band.
///
/// [Security Schemes]: Components::security_schemes
/// [Components Object]: Components
/// [OpenAPI Object]: Spec
/// [Operation Object]: Operation
pub type SecurityRequirement = HashMap<String, Vec<String>>;

/// Any value.
///
/// Untyped value.
pub type Any = serde_json::Value; // TODO: create our own type.
