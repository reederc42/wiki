const DEFAULT_OUTPUT: &str = "wiki/src/mime_types/json";
const DEFAULT_SOURCE_URL: &str = 
    "https://raw.githubusercontent.com/nginx/nginx/master/conf/mime.types";

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Output file for MIME types
    #[arg(short, long, default_value = DEFAULT_OUTPUT)]
    output: String,

    /// Source URL from NGINX
    #[arg(
        short,
        long,
        default_value = DEFAULT_SOURCE_URL,
    )]
    source: String,
}

pub fn cmd(args: Args) {
}

#[cfg(test)]
mod tests {
}
