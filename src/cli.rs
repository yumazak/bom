use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(SubCommand::with_name("add")// サブコマンドを定義
            .about("add folder to list")         // このサブコマンドについて
            .arg(Arg::with_name("url")       // フラグを定義
                .help("folder url")     // ヘルプメッセージ
                .required(true)
                .takes_value(true) 
            )
            .arg(Arg::with_name("name")       // フラグを定義
                .help("boilerplate name")     // ヘルプメッセージ
                .takes_value(true) 
            )
        )
}

fn not_at(v: String) -> Result<(), String> {
    if v.contains("@") {
        return Err(String::from("The value can not contain the '@' character"));
    }
    Ok(())
}