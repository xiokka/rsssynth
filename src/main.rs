use chrono::DateTime;
use chrono::NaiveDate;
use std::fs::File;
use std::io::BufReader;
use rss::Channel;
use rss::Item;


fn items_to_html(items: Vec<Item>) -> String {
    let mut feed = String::new();

    // Default values
    let default_title = "Untitled".to_string();
    let default_item_title = "No Title".to_string();
    let default_pub_date = "No Date".to_string();
    let default_description = "No Description".to_string();

    for item in items {
        let item_author = item.author.as_ref().unwrap_or(&default_item_title);

        let item_title = item.title.as_ref().unwrap_or(&default_item_title);
        let pub_date = item.pub_date.as_ref().unwrap_or(&default_pub_date);
        let description = item.description.as_ref().unwrap_or(&default_description);

        let mut this_item = format!(
            "<div class=\"item\"><h2>{}</h2> <h2>{}</h2> <p><i>{}</i></p> <p>{}</p>",
            item_author, item_title, pub_date, description
        );

        if item.enclosure != None {
		let enclosure = item.enclosure.unwrap();
		let mut length:u64 = enclosure.length.parse().unwrap(); // bytes
		length = (length / 1024); // kb
		let mut unit:String = "KB".to_string();
		if length > 1024 {
			length = length / 1024; // mb
			unit = "MB".to_string();
		}
		this_item.push_str(&format!("<a href=\"{}\">📎See attached media [{} {} {}]</a>", enclosure.url.to_string(), length.to_string(), unit,  enclosure.mime_type.to_string()));
        }
	this_item.push_str("</div>");

        feed.push_str(&this_item);
	
    }

    feed
}

    const microblog_html:&str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Microblog</title>
    <style>
	body {
background-color: #feffee;
opacity: 1;
background-image:  linear-gradient(#6a0005 8px, transparent 8px), linear-gradient(90deg, #6a0005 8px, transparent 8px), linear-gradient(#6a0005 4px, transparent 4px), linear-gradient(90deg, #6a0005 4px, #feffee 4px);
background-size: 200px 200px, 200px 200px, 40px 40px, 40px 40px;
background-position: -8px -8px, -8px -8px, -4px -4px, -4px -4px;

	    color: #444;
	    font-family: monospace;
	    margin: 0;
	    padding: 20px;
	}
header {
    padding: 4px;
    background-color: #6a0005; 
/*    border: 1px solid black;*/
    text-align: center;
    color: white;
    font-size: 14px;
    font-weight: bold;
    width: 100%;
}

h2 {
	color: #6a0005;
	font-size: 12px;
}

p {
	font-size: 12px;
}
.container {
    outline: 1px solid black;
    display: flex;
    flex-wrap: wrap; 
    justify-content: space-between; 
    margin: 10px auto; 
    align-items: flex-start; 
    gap: 10px;
    max-width: 1200px;
    background-color: #feffee;
    border: 1px solid #6a0005;
}

.item {
    outline: 1px solid black;
    width: calc(33.333% - 20px); /* Three items per row with spacing */
    box-sizing: border-box; 
    float:left;
    color: black;
    background-color: white; 
    border: 1px solid #6a0005;
    /*border-radius: 5px;*/
    padding: 15px;
    margin: 10px;
}

/* Responsive adjustments */
@media (max-width: 768px) {
    .item {
        width: calc(50% - 20px); /* Two items per row on smaller screens */
    }
}

@media (max-width: 480px) {
    .item {
        width: calc(100% - 20px); /* One item per row on very small screens */
    }
}

a {
    color: #4CAF50;
    text-decoration: none;
}
a:hover {
    text-decoration: underline;
}


    </style>
</head>
<body>

<div class="container">
	<header>
	    Microblog
	</header>
	<div class="posts">

"#;

use std::fs;
use std::path::Path;
use std::env;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() <= 1 {
		println!("No arguments provided. Please direct me to a feeds directory.");
		return Ok(());
	}
	let binding = args[1].to_string();
	let folder_path = std::path::Path::new(&binding); // Store the first argument in the folder_path variable
	

    // Read the directory entries
    let entries = fs::read_dir(folder_path)?;

    // Iterate over the entries
    let mut all_entries:Vec<Item> = vec![];
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check if the path is a file and has a .xml extension
        if path.is_file() && path.extension().map(|s| s == "xml").unwrap_or(false) {
	    let file = File::open(path).unwrap();
	    let mut channel = Channel::read_from(BufReader::new(file)).unwrap();
	    for i in 0..channel.items.len() {
		channel.items[i].author = Some(format!("<a href=\"{}\">{}</a>", channel.link, channel.title.to_string()));
	    }


	if let Some(ref image) = channel.image {
	        for item in &mut channel.items {
	            if let Some(author) = item.author.as_mut() {
	                // Append the image tag to the author's string because yeah sorry it's 3am i am not thinking of a better solution rn
	                author.insert_str (0, &format!("<img src=\"{}\" width=\"64\" height=\"64\" style=\"border-radius: 50%; float: left; padding: 10px\">", channel.image.clone().unwrap().url));
	            }
	        }
	}

	    all_entries.extend(channel.items);
        }
    }


    // Sort items by pub_date in descending order
    all_entries.sort_by(|a, b| {
        let date_a = a.pub_date.as_ref().and_then(|date| DateTime::parse_from_str(date, "%a, %d %b %Y %H:%M:%S %z").ok());
        let date_b = b.pub_date.as_ref().and_then(|date| DateTime::parse_from_str(date, "%a, %d %b %Y %H:%M:%S %z").ok());

        match (date_a, date_b) {
            (Some(d_a), Some(d_b)) => d_b.cmp(&d_a), // Reverse the order for descending sort
            (None, Some(_)) => std::cmp::Ordering::Less, // `None` comes before `Some`
            (Some(_), None) => std::cmp::Ordering::Greater, // `Some` comes after `None`
            (None, None) => std::cmp::Ordering::Equal, // Both are `None`
        }
    });



    println!("{}", microblog_html);
    println!("{}", items_to_html(all_entries));
    println!("</div> </div> </body> </html>");

    Ok(())
}
