use clap::ValueEnum;
use sophia::parser;
use sophia::serializer;
use std::fmt::Error;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

#[derive(Clone, Debug, ValueEnum)]
pub(crate) enum GraphFormat {
    #[clap(alias = "ttl")]
    Turtle,
    #[clap(alias = "nt", alias = "ntriples")]
    NTriples,
    #[clap(alias = "xml", alias = "rdf/xml")]
    RdfXml,
}

impl FromStr for GraphFormat {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ntriples" | "nt" | "n-triples" => Ok(GraphFormat::NTriples),
            "xml" | "rdf/xml" | "rdf-xml" => Ok(GraphFormat::RdfXml),
            "ttl" | "turtle" => Ok(GraphFormat::Turtle),
            _ => Err(Error),
        }
    }
}
// Call the appropriate parser based on the input RDF format.
// For now, just wrapping the oxigraph parser, but this could
// be extended to support other parsers.
pub fn parse_any_rdf<R: BufRead>(
    src: R,
    format: GraphFormat,
) -> Result<TripleReader<R>, ParseError> {
    GraphParser::from_format(format).read_triples(src)
}

pub fn serialize_any_rdf<W: Write>(
    dest: W,
    format: GraphFormat,
) -> Result<TripleWriter<W>, io::Error> {
    GraphSerializer::from_format(format).triple_writer(dest)
}
