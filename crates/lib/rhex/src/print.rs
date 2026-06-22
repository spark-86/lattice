use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

use crate::Rhex;

impl Rhex {
    pub fn pretty_print(&self) -> String {
        let magic = match self.magic.as_slice() {
            b"RHEX\x00\x01" => "V1",
            b"RHEX\x00\x02" => "V2",
            _ => "Unknown",
        };
        let prev_hash = match self.intent.prev {
            Some(prev) => URL_SAFE_NO_PAD.encode(prev),
            None => "None".to_string(),
        };
        let scope = match self.intent.scope.as_str() {
            "" => "(root)".to_string(),
            _ => self.intent.scope.clone(),
        };
        let author = URL_SAFE_NO_PAD.encode(self.intent.author);
        let usher = URL_SAFE_NO_PAD.encode(self.intent.usher);
        let schema = match &self.intent.schema {
            Some(schema) => schema,
            None => &"None".to_string(),
        };
        let spacial = match &self.context.s {
            Some(s) => format!(
                "\"{}\": {}",
                s.s_ref,
                URL_SAFE_NO_PAD.encode(s.s_data.clone())
            ),
            None => "None".to_string(),
        };
        let mut sigs = Vec::new();
        for sig in self.sigs.iter() {
            sigs.push(sig.print());
        }
        let sigs = format!("\n\t\t{}", sigs.join("\n\t\t"));
        let curr = match self.curr {
            Some(curr) => URL_SAFE_NO_PAD.encode(curr),
            None => "None".to_string(),
        };

        format!(
            "Rhex: {{
    \tmagic: {},
    \tintent: {{
    \t\tprev: {},
    \t\tscope: {},
    \t\tauthor: {},
    \t\tusher: {},
    \t\trt: {},
    \t\tschema: {},
    \t\tdata: {},
    \t}},
    \tcontext: {{
    \t\tat: {},
    \t\ts: {},
    \t}},
    \tsigs: {},
    \tcurr: {},
}}",
            magic,
            prev_hash,
            scope,
            author,
            usher,
            self.intent.rt,
            schema,
            self.intent.data.print(),
            self.context.at,
            spacial,
            sigs,
            curr
        )
    }
}
