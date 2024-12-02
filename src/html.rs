pub const microblog_html:&str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>rsssynth</title>
    <style>
        .item img {
                max-width: 100%;
        }

        body {
            color: #6a0005;
            font-family: monospace;
            margin: 0;
        }
header {
    padding: 4px;
    box-sizing: border-box;
    background-color: #6a0005;
    outline: 1px solid black;
    text-align: center;
    color: white;
    font-size: 14px;
    font-weight: bold;
    width: 100%;
    text-align: center;
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
    justify-content: space-between;
    margin: 0 10px 10px;
    align-items: flex-start; 
    gap: 10px;
    background-color: #feffee;
    border: 1px solid #6a0005;
    max-width: 1000px;
}

.item {
    outline: 1px solid black;
    box-sizing: border-box;
    color: black;
    background-color: white;
    border: 1px solid #6a0005;
    /*border-radius: 5px;*/
    padding: 15px;
    margin: 10px;
    flex-direction: column;
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
	<div>
        <header>
                rsssynth
        </header>
        <div class="posts">
		{CONTENT}
	</div>
	</div>	
</div>
</body>
</html>
"#;
