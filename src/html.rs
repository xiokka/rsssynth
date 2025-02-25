pub const microblog_html:&str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>rsssynth</title>
    <style>

@font-face {
    font-family: 'DotGothic16';
    src: url('/font/DotGothic16-Regular.ttf') format('truetype'),
}

        .item img {
                max-width: 100%;
        }

        body {
            color: #6a0005;
            font-family: 'DotGothic16';
            margin: 0;
	    line-height: 1;
	    font-size: 13px;
        }
header {
    width: 100%;
    padding: 4px;
    box-sizing: border-box;
    background-color: #6a0005;
    outline: 1px solid black;
    color: white;
    font-size: 13px;
    font-weight: bold;
    text-align: center;
}

h2 {
        color: #6a0005;
        font-size: 13px;
}

i {
font-style: normal; /* Ensures text is not italicized */
}
p {
	font-style: normal; /* Ensures text is not italicized */
        font-size: 13px;
}

.container {
    outline: 1px solid black;
    display: flex;
    justify-content: space-between;
    margin: 0 10px 10px;
    align-items: flex-start; 
    gap: 10px;
    background-color: white;
    max-width: 1000px;
}

.item {
    border-bottom: 1px solid #000; /* 2px thickness, solid line, black color */
    box-sizing: border-box;
    color: black;
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
	<div style="width: 100%">
        <header>
                Xiokka's Feed
        </header>
        <div class="posts">
		{CONTENT}
	</div>
	</div>	
</div>
</body>
</html>
"#;
