use fluent::{FluentBundle, FluentResource};
use unic_langid::langid;

pub const AVAILABLE_LANGS: &[&str] = &["en", "fr"];

const EXAMPLE_LIST_EN: &[&str] = &["insect_bite", "rain", "flat_earth"];
const EXAMPLE_LIST_FR: &[&str] = &[];

pub struct Lang {
    pub(crate) name: String,
    pub(crate) bundle: FluentBundle<FluentResource>,
    pub(crate) examples: &'static [&'static str],
}

impl Lang {
    pub fn load(name: &str) -> Result<Lang, ()> {
        let (id, ftl, examples) = match name {
            "en" => (
                langid!("en-US"),
                include_str!("../lang/en.ftl"),
                EXAMPLE_LIST_EN,
            ),
            "fr" => (
                langid!("fr-FR"),
                include_str!("../lang/fr.ftl"),
                EXAMPLE_LIST_FR,
            ),
            _ => return Err(()),
        };
        let resource =
            FluentResource::try_new(ftl.into()).expect("Failed to parse the FTL resource.");
        let mut bundle = FluentBundle::new(&[id]);
        bundle
            .add_resource(resource)
            .expect("Failed to add the FTL resource to the bundle.");
        Ok(Lang {
            name: name.into(),
            bundle,
            examples,
        })
    }
}

#[macro_export]
macro_rules! lang {
    ($lang:expr, $msgid:expr) => { {
        let msg = $lang.bundle.get_message($msgid)
            .unwrap_or_else(|| panic!("Message \"{}\" does not exist.", $msgid));
        let mut errors = Vec::new();
        let pattern = msg.value.unwrap_or_else(|| panic!("Message \"{}\" has no value.", $msgid));
        let value = $lang.bundle.format_pattern(&pattern, None, &mut errors);
        for error in errors {
            use stdweb::console;
            console!(log, format!("Translation error in message \"{}\": {:?}", $msgid, error));
        }
        value.to_string()
    } };
    ($lang:expr, $msgid:expr, $($var:ident = $val:expr),* ) => { {
        use fluent::{FluentArgs, FluentValue};
        let msg = $lang.bundle.get_message($msgid)
            .unwrap_or_else(|| panic!("Message \"{}\" does not exist.", $msgid));
        let mut errors = Vec::new();
        let pattern = msg.value.unwrap_or_else(|| panic!("Message \"{}\" has no value.", $msgid));
        let mut args = FluentArgs::new();
        $(
            args.insert(stringify!($var), FluentValue::from($val));
        )*
        let value = $lang.bundle.format_pattern(&pattern, Some(&args), &mut errors);
        for error in errors {
            use stdweb::console;
            console!(log, format!("Translation error in message \"{}\": {:?}", $msgid, error));
        }
        value.to_string()
    } };
}
