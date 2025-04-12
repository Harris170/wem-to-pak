use std::fs;
use std::io;
use std::process::Command;

fn main() -> io::Result<()>
{
    let args:Vec<String> = std::env::args().collect();
    let mut output_pak_name = "LunaUlt_Song_P.pak";
    let current_directory = std::env::current_dir().expect("Error: Unable to get current directory");
    let input_wem_file = find_wem_file(&current_directory)
        .expect("Error: No .wem file found in the current directory.");

    let soundmod_folder = "apps\\soundMod";
    let bnksfx_folder = "apps\\soundMod\\bnk_sfx_1031001";
    let sound_file_editor_exe = "apps\\soundMod\\SoundFileEditor.exe";
    let bnksfx_file = "apps\\soundMod\\Output\\bnk_sfx_1031001.bnk";

    let u4pakc_folder = "apps\\u4pakc";
    let wwise_audio_folder = "apps\\u4pakc\\Marvel\\Content\\WwiseAudio";
    let compress_bat = "apps\\u4pakc\\compress.bat";
    let marvel_pak = "apps\\u4pakc\\Marvel.pak";

    if args.len() < 2
    {
        println!("Info: No output file name provided. Defaulting to \"{output_pak_name}\". Provide custom output name as '{} CustomFileName_P.pak'", args[0]);
    }

    if args.len() == 2
    {
        output_pak_name = args[1].as_str();
    }

    if !find_dir(soundmod_folder)
    {
        error_exit(format!("Error: Destination '{}' not found.", soundmod_folder));
    }

    let new_wem_file = current_directory.join("LunaUlt.wem");
    match fs::rename(input_wem_file.clone(), new_wem_file.clone())
    {
        Ok(_) => println!("Ok: Renamed {} to LunaUlt.wem", input_wem_file.display()),
        Err(e) => error_exit(format!("{e}")),
    }
    let input_wem_file = new_wem_file;

    match copy_file_to_directory(input_wem_file.to_str().expect(""), bnksfx_folder)
    {
        Ok(_) => (),
        Err(e) => error_exit(e),
    }

    println!("Ok: Starting SoundFileEditor.exe");
    if !find_file(sound_file_editor_exe)
    {
        error_exit(format!("Error: While finding SoundFileEditor.exe: File not found\n\tPut SoundFileEditor.exe in the soundMod folder\n\tThe file path should look like \"{sound_file_editor_exe}\""));
    }

    let mut child = Command::new(sound_file_editor_exe)
        .current_dir(soundmod_folder)
        .spawn()?;

    std::thread::sleep(std::time::Duration::from_secs(4)); println!("\n");
    match child.kill()
    {
        Ok(_) => (),
        Err(e) => println!("Error: While closing SoundFileEditor.exe: {e}"),
    }
    println!("Info: Ran SoundFileEditor.exe");

    if !find_dir(u4pakc_folder)
    {
        error_exit(format!("Error: Destination '{}' not found.", u4pakc_folder));
    };

    match copy_file_to_directory(bnksfx_file, wwise_audio_folder)
    {
        Ok(_) => (),
        Err(e) => error_exit(format!("{e}")),
    }

    if !find_file(compress_bat)
    {
        error_exit(format!("Error: While finding compress.bat: File not found.\n\tPut compress.bat file in u4pakc folder.\n\tThe file path should look like: \"{compress_bat}\""));
    }

    println!("Ok: Starting compress.bat");
    let status = Command::new(compress_bat)
        .current_dir(u4pakc_folder)
        .status()?; println!("\n");

    if status.success() { println!("Info: Ran compress.bat"); }
    else
    {
        println!("Error: While running compress.bat: Could not convert to Marvel.pak");
        std::process::exit(-1);
    }

    if !find_file(marvel_pak)
    {
        error_exit(format!("Error: File {marvel_pak}: File not found\n\tThis message should be unreachable.\n\tEither the compress.bat failed to convert bnk_sfx_1031001.bnk to {marvel_pak} successfully or somehow it has been removed or renamed from the \"{u4pakc_folder}\" folder"));
    }

    match fs::rename(marvel_pak, output_pak_name)
    {
        Ok(_) => println!("Ok: Renamed Marvel.pak to {output_pak_name}"),
        Err(e) => println!("Error: While renaming Marvel.pak: {e}"),
    };

    println!("\n-------------------------\n");
    println!("\tCLEANING UP");
    println!("\n-------------------------\n");

    match fs::remove_file(bnksfx_folder.to_owned() + "\\LunaUlt.wem")
    {
        Ok(_) => println!("Removed bnk_sfx_1031001.bnk from '{bnksfx_folder}'"),
        Err(e) => eprintln!("Error: While removing bnk_sfx_1031001.bnk from '{bnksfx_folder}': {e}"),
    }

    match fs::remove_file(soundmod_folder.to_owned() + "\\Output\\bnk_sfx_1031001.bnk")
    {
        Ok(_) => println!("Removed bnk_sfx_1031001.bnk from '{}'", soundmod_folder.to_owned() + "\\Output"),
        Err(e) => eprintln!("Error: While removing bnk_sfx_1031001.bnk from {}: {e}", soundmod_folder.to_owned() + "\\Output"),
    }

    match fs::remove_file(wwise_audio_folder.to_owned() + "\\bnk_sfx_1031001.bnk")
    {
        Ok(_) => println!("Removed bnk_sfx_1031001.bnk from '{}'", wwise_audio_folder),
        Err(e) => eprintln!("Error: While removing bnk_sfx_1031001.bnk: {e}"),
    }

    println!("\n-------------------------\n");
    println!("\tSUCCESS");
    println!("\n-------------------------\n");

    Ok(())
}

fn find_file(path: &str) -> bool
{
    return std::path::Path::new(path).exists();
}

fn find_dir(path: &str) -> bool
{
    return std::path::Path::new(path).exists();
}

fn copy_file_to_directory(source: &str, destination: &str) -> Result<(), String> {
    if !find_dir(destination)
    {
        return Err(format!("Error: Destination '{}' not found.", destination));
    }

    if !find_file(source)
    {
        return Err(format!("Error: Source '{}' not found.", source));
    }

    let source_file_name = std::path::Path::new(source)
        .file_name()
        .ok_or_else(|| format!("Error: Could not extract the file name from '{}'.", source))?;
    let destination_path: std::path::PathBuf = std::path::Path::new(destination).join(source_file_name);

    match fs::copy(source, &destination_path)
    {
        Ok(_) =>
        {
            println!("Ok: File copied successfully from '{}' to '{}'.", source, destination_path.display());
            Ok(())
        }
        Err(e) => Err(format!(
            "Error: Failed to copy '{}' to '{}': {}",
            source,
            destination_path.display(),
            e
        )),
    }
}

fn find_wem_file(directory: &std::path::PathBuf) -> Option<std::path::PathBuf>
{
    fs::read_dir(directory)
        .ok()?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path()) // Get the file paths
        .find(|path| path.extension().map_or(false, |ext| ext == "wem"))
}

fn error_exit(message: String)
{
    eprintln!("{message}");
    println!("\n-------------------------\n");
    println!("\tFAILURE");
    println!("\n-------------------------\n");

    std::process::exit(-1);
}
