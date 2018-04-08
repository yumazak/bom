use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(SubCommand::with_name("add")
            .about("add folder to list")
            .arg(Arg::with_name("url")
                .help("folder url")
                .required(true)
                .takes_value(true) 
            )
            .arg(Arg::with_name("name")
                .help("boilerplate name")
                .takes_value(true) 
            )
        )
        .subcommand(SubCommand::with_name("rm")
            .about("remove boilerplate")
            .arg(Arg::with_name("name")
                .help("boilerplate name")
                .required(true)
                .takes_value(true) 
            )
            // .arg(Arg::with_name("project_name")
            //     .help("project name")
            //     .takes_value(true) 
            // )
            // .arg(Arg::with_name("current_flag")
            //     .help("init current directory")                // ヘルプメッセージ
            //     .short("c")                         // ショートコマンド
            //     .long("cur")                       // ロングコマンド
            // )
        )
}