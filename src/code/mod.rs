//! Code generation.

use std::fmt::Write;
use std::io;

use crate::Spec;

mod rust;
pub use rust::Rust;

/// Code generator.
pub struct Generator<L> {
    language: L,
}

impl<L> Generator<L>
where
    L: Language,
{
    /// Create a new code `Generator`.
    pub const fn new(language: L) -> Generator<L> {
        Generator { language }
    }

    /// Write `spec` to `out`.
    ///
    /// `warnings` will be filled with, well, warnings.
    ///
    /// # Notes
    ///
    /// Writes are not buffered to `out`, so you mind want to use a
    /// [`BufWriter`] or similar.
    ///
    /// [`BufWriter`]: std::io::BufWrite
    pub fn write_to<W>(&self, spec: &Spec, mut out: W, warnings: &mut Vec<String>) -> io::Result<()>
    where
        W: io::Write,
    {
        self.write_module_docs(spec, &mut out)?;
        if spec.json_schema_dialect.is_some() {
            warnings.push("$root.jsonSchemaDialect not supported".to_owned());
        }

        // TODO: use to set base URL of the client.
        // Maybe a list of known servers?
        //pub servers: Vec<Server>,

        //pub paths: Paths,

        //pub components: Components,

        if !spec.webhooks.is_empty() {
            warnings.push("$root.webhooks not supported".to_owned());
        }

        if !spec.security.is_empty() {
            warnings.push("$root.security not supported".to_owned());
        }

        // TODO: use `tags` to split the data structures and routes into new
        // files/modules.

        out.flush()
    }

    /// Writes the module documentation, roughly the following.
    ///
    /// ```text
    /// ${info.title}
    ///
    /// ${info.description || info.summary}.
    ///
    /// <${external_docs.url}>: ${external_docs.description}>
    ///
    /// Version: ${info.version}
    /// Contact: ${info.contact.name} <${info.contact.email}>, ${info.contact.url}
    /// License: ${info.license.name} (${info.license.identifier}), ${info.license.url}
    /// Terms of Service: ${info.terms_of_service}
    /// ```
    fn write_module_docs<W>(&self, spec: &Spec, out: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        let info = &spec.info;
        let mut docs = String::new();
        docs.push_str(&info.title);

        if let Some(desc) = info.description.as_deref().or(info.summary.as_deref()) {
            docs.push_str("\n\n");
            docs.push_str(desc);
            if !desc.ends_with('.') {
                docs.push('.');
            }
        }

        if let Some(external_docs) = spec.external_docs.as_ref() {
            docs.push_str("\n\n");

            let desc = if let Some(desc) = external_docs.description.as_ref() {
                desc.strip_suffix('.').unwrap_or(desc)
            } else {
                "More documentation at"
            };
            write!(docs, "{}: <{}>.", desc, external_docs.url).unwrap();
        }

        docs.push_str("\n\nVersion: ");
        docs.push_str(&info.version);

        if let Some(contact) = info.contact.as_ref() {
            if let Some(name) = contact.name.as_ref() {
                docs.push_str("\nContact: ");
                docs.push_str(name);
            }

            if let Some(email) = contact.email.as_ref() {
                let has_name = contact.name.is_some();
                docs.push_str(if has_name { " <" } else { "\n" });
                docs.push_str(email);
                if has_name {
                    docs.push('>')
                }
            }

            if let Some(email) = contact.email.as_ref() {
                let a = contact.name.is_some() || contact.email.is_some();
                docs.push_str(if a { ", " } else { "\n" });
                docs.push_str(email);
            }
        }

        if let Some(license) = info.license.as_ref() {
            docs.push_str("\nLicense: ");
            docs.push_str(&license.name);

            if let Some(identifier) = license.identifier.as_ref() {
                docs.push_str(" (");
                docs.push_str(identifier);
                docs.push(')');
            }

            if let Some(url) = license.url.as_ref() {
                docs.push(' ');
                docs.push_str(url);
            }
        }

        if let Some(tos) = info.terms_of_service.as_deref() {
            docs.push_str("\nTerms of Service: ");
            docs.push_str(&tos);
        }

        self.language.module_docs(&docs, out)
    }
}

pub trait Language {
    /// Write module, or file, documentation. `docs` may contain [CommonMark
    /// syntax]. Defaults to no documentation.
    ///
    /// [CommonMark syntax]: https://spec.commonmark.org
    fn module_docs<W: io::Write>(&self, docs: &str, out: W) -> io::Result<()> {
        let _ = docs;
        let _ = out;
        Ok(())
    }

    // TODO.
}
