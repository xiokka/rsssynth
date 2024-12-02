mod html;
use crate::html::microblog_html;

use chrono::DateTime;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rss::Channel;
use rss::Item;
use std::io::Write;
	
mod fetch;
use crate::fetch::*;

fn items_to_html(items: &Vec<Item>) -> String {
    let mut feed = String::new();

    // Default values
    let default_title = "Untitled".to_string();
    let default_item_title = "No Title".to_string();
    let default_pub_date = "No Date".to_string();
    let default_description = "No Description".to_string();

for i in 0..items.len() {
    let item = &items[i]; // Access the item by index
    let item_author = item.author.as_ref().unwrap_or(&default_item_title);
    let item_title = item.title.as_ref().unwrap_or(&default_item_title);
    let pub_date = item.pub_date.as_ref().unwrap_or(&default_pub_date);
    let description = item.description.as_ref().unwrap_or(&default_description);

    let mut this_item = format!(
        "<div class=\"item\"><h2>{}</h2> <h2>{}</h2> <p><i>{}</i></p> <p>{}</p>",
        item_author, item_title, pub_date, description
    );

    if let Some(enclosure) = &item.enclosure {
        let mut length: u64 = enclosure.length.parse().unwrap_or(0); // bytes
        length /= 1024; // Convert to KB
        let mut unit = "KB".to_string();
        
        if length > 1024 {
            length /= 1024; // Convert to MB
            unit = "MB".to_string();
        }
        
        this_item.push_str(&format!(
            "<a href=\"{}\">📎See attached media [{} {}]</a>",
            enclosure.url,
            length,
            unit
        ));
    }
    
    this_item.push_str("</div>");
    feed.push_str(&this_item);
}
    feed
}


pub fn merge_and_sort(channels: &Vec<Channel>) -> Vec<Item> {
	let feeds = channels.clone();
        let mut all_entries:Vec<Item> = vec![];

        for mut channel in feeds {
		let mut author = String::new();
        	if let Some(ref image) = channel.image {
			author = format!("<img src=\"{}\" width=\"64\" height=\"64\" style=\"border-radius: 50%; float: left; padding: 10px\">", channel.image.clone().unwrap().url);
            	}
		author.push_str(&format!("<a href=\"{}\">{}</a>", channel.link.to_string(), channel.title.to_string()));
		for i in 0..channel.items.len() {
			channel.items[i].author = Some(author.to_string());
		}
		all_entries.extend(channel.items.clone());
	}

        // Sort items by pub_date in descending order
        all_entries.sort_unstable_by(|a, b| {
                let date_a = a.pub_date.as_ref().and_then(|date| DateTime::parse_from_rfc2822(date).ok());
                let date_b = b.pub_date.as_ref().and_then(|date| DateTime::parse_from_rfc2822(date).ok());
                match (date_a, date_b) {
                        (Some(d_a), Some(d_b)) => d_b.cmp(&d_a), // Reverse the order for descending sort
                        (None, Some(_)) => std::cmp::Ordering::Less, // `None` comes before `Some`
                        (Some(_), None) => std::cmp::Ordering::Greater, // `Some` comes after `None`
                        (None, None) => std::cmp::Ordering::Equal, // Both are `None`
                }
        });
        return all_entries;
}

use std::fs;
use std::path::Path;
use std::env;
use std::io;
use std::cmp::Ordering;
fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() <= 1 {
		println!("Invalid arguments. Correct usage: rsssynth <path>");
		return Ok(());
	}

    let path = args[1].to_string();
    let path = std::path::Path::new(&path);

    let mut channels:Vec<Channel> = match path {
	_ if path.is_dir() => grab_feeds_from_directory(&path),
	_ if path.is_file() => {
		let file = File::open(&path).map_err(|e| {
			io::Error::new(
				e.kind(),
				format!("Failed to open the file at {}: {}.", path.display(), e),
			)
		})?;
		let reader = io::BufReader::new(file);
		// Initialize a Vec<&str> to store each line from the file
		let mut url_vec: Vec<String> = Vec::new();
		// Iterate over each line in the file	
		for line in reader.lines() {
			// Unwrap the line and push it to the vector
			match line {
				Ok(line_str) => url_vec.push(line_str),
				Err(e) => eprintln!("Error reading line: {}", e),
			}
		}
		fetch_feeds_concurrently(&url_vec)
		}
	_ => { 
	     eprintln!("Error: Path is neither a directory nor a file.");
	     Vec::new()
	},
    };

    // Create index.html file
    let all_entries = merge_and_sort(&channels);
    let mut base_html = microblog_html.to_string();
    let items_html = items_to_html(&all_entries);
    base_html = base_html.replace("{CONTENT}", &items_html);
    let mut file = File::create("index.html")?;
    file.write_all(base_html.as_bytes())?;

    // Create a new vector with a copy of the first element
    let latest_entry = if let Some(first_element) = all_entries.get(0).cloned() {
        vec![first_element]
    } else {
        Vec::new() // Return an empty vector if original_vector is empty
    };

    // Create last_update file
    let mut base_html = microblog_html.to_string();
    let items_html = items_to_html(&latest_entry);
    base_html = base_html.replace("{CONTENT}", &items_html);
    let mut file = File::create("last_update.html")?;
    file.write_all(base_html.as_bytes())?;

    println!("HTML files generated.");
    Ok(())
}
