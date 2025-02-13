use std::{collections::HashMap, os::unix::process::CommandExt, process::{Command, Stdio}};
fn main() 
{
    println!("Hello,!");
        
    let mut command = Command::new("xprop");

    let output = command.output().unwrap().stdout;

    // To string
    let string_output = String::from_utf8(output).unwrap().clone();

    XpropOutput::parse_xprop_output(
                string_output.as_str(),
                vec!["WM_CLASS(STRING)".to_string()]
    );

    // println!("{:?}", String::from_utf8(output.unwrap().stdout));
}



/// A struct used to store the information found in the output of the xprop command
struct XpropOutput
{
    wm_informations: HashMap<String, Vec<String>>
}

impl XpropOutput 
{
    /// Parses the xprop output and locates the values of the located keyword
    fn parse_xprop_output(output: &str, mut query_infos : Vec<String>) -> HashMap<String, Vec<String>>
    {
        let mut res: HashMap<String, Vec<String>> = HashMap::new();

        let mut curr_info_name = String::new();

        let mut is_string = false;
        let mut index;
        let mut last_word: String = String::new();
        for c in output.chars()
        {
            if !is_string && (c.eq(&'=') || c.eq(&'\n') || c.eq(&':'))
            {
                if curr_info_name.len() != 0 && c.eq(&'=') 
                {
                    //TODO replace d by current_info_name
                    res.get(&String::from("d")).unwrap().push(last_word.to_string());
                    curr_info_name.clear();
                }
                if last_word.len() != 0 
                {
                    // Remove any white space
                    last_word = last_word.trim().to_string();
                    println!("=> {}", last_word);

                    // if this was one of the search information
                    index = query_infos.iter().position(|w| w == &last_word);
                    if let Some(i) = index
                    {
                        println!("{:?}", i);
                        println!("\t EUREKA !");
                        // We don't have to look for it anymore
                        // And we store this info to know that the value after the words
                        // after the equal are what we want
                        curr_info_name = query_infos.remove(i);

                        res.insert(curr_info_name.clone(), Vec::new());
                    }
                    // If the word is part of the query info, we need to look for the values after the '=' char.
                }  
                last_word.clear();
            }
            // If a string is opening or closing
            else if c.eq(&'\"') 
            {
                is_string = !is_string;
                // But we don't push the \" char
            }
            else if !c.eq(&'\t')
            {
                last_word.push(c);    
            }
        }
        return res;         
    }


    /// Creates a new [XpropOutput] struct. 
    /// Will force the user to click on a window, in order to execute the xprop command.
    /// 
    /// # Arguments
    ///  
    /// * `query_info` - The name of the fields to store in the resulting [XpropOutput] struct.
    pub fn new(query_info : Vec<String>) -> Self
    {
        let res = HashMap::new();
        XpropOutput
        {
            wm_informations: res
        }
    }
}


// xwininfo