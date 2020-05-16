use std::fs::File;
use std::io::Read;

fn main() {


    println!("Init config for pocketsphinx");


    let ps_config = pocketsphinx::CmdLn::init(true, &["pocketsphinx",
        "-hmm", "/usr/local/share/pocketsphinx/model/en-us/en-us",
        "-lm", "/usr/local/share/pocketsphinx/model/en-us/en-us.lm.bin",
        "-dict", "/usr/local/share/pocketsphinx/model/en-us/cmudict-en-us.dict",
        ]);
    let ps_decoder = pocketsphinx::PsDecoder::init(ps_config.unwrap());



    let mut f = File::open(&"./Audiospur.raw").unwrap();
    let mut buffer = [0; 2048];
    let mut samples : [i16; 1024];

    ps_decoder.start_utt(Some("something"));
    loop {    
        let n = f.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        samples = unsafe {std::mem::transmute(buffer)};
        ps_decoder.process_raw(&mut samples, false, false);
    }
    ps_decoder.end_utt();
    match ps_decoder.get_hyp() {
            None => println!("Not recognized"),
            Some((hyp, _utt_id, _score)) => println!("Recognized: {} - SCORE {}", hyp, _score),
    }


}

