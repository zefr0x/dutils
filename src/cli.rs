use crate::common::{Handler, MimeOrExtension, UserPath};

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None, disable_help_subcommand = true)]
pub enum Cmd {
    /// List default apps and the associated handlers
    List {
        #[clap(long, short)]
        all: bool,
    },

    /// Open a path/URL with its default handler
    Open {
        #[clap(required = true)]
        paths: Vec<UserPath>,
    },

    /// Set the default handler for mime/extension
    Set {
        mime: MimeOrExtension,
        handler: Handler,
    },

    /// Unset the default handler for mime/extension
    Unset { mime: MimeOrExtension },

    /// Launch the handler for specified extension/mime with optional arguments
    Launch {
        mime: MimeOrExtension,
        args: Vec<UserPath>,
    },

    /// Get handler for this mime/extension
    Get {
        #[clap(long)]
        json: bool,
        mime: MimeOrExtension,
    },

    /// Add a handler for given mime/extension
    /// Note that the first handler is the default
    Add {
        mime: MimeOrExtension,
        handler: Handler,
    },

    #[command(hide = true)]
    Autocomplete {
        #[clap(short)]
        desktop_files: bool,
        #[clap(short)]
        mimes: bool,
    },
}
