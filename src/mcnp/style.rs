pub fn get_css() -> String {
    return String::from(r" 
    <style type='text/css'>

    /*----------------------------------------------------------------------
        CSS RESET
        http://meyerweb.com/eric/tools/css/reset/
        v2.0 | 20110126
        License: none (public domain)
    ----------------------------------------------------------------------*/
    
    html, body, div, span, applet, object, iframe,
    h1, h2, h3, h4, h5, h6, p, blockquote, pre,
    a, abbr, acronym, address, big, cite, code,
    del, dfn, em, img, ins, kbd, q, s, samp,
    small, strike, strong, sub, sup, tt, var,
    b, u, i, center,
    dl, dt, dd, ol, ul, li,
    fieldset, form, label, legend,
    table, caption, tbody, tfoot, thead, tr, th, td,
    article, aside, canvas, details, embed,
    figure, figcaption, footer, header, hgroup,
    menu, nav, output, ruby, section, summary,
    time, mark, audio, video {
        margin: 0;
        padding: 0;
        border: 0;
        font-size: 100%;
        font: inherit;
        vertical-align: baseline;
    }
    /* HTML5 display-role reset for older browsers */
    article, aside, details, figcaption, figure,
    footer, header, hgroup, menu, nav, section {
        display: block;
    }
    body {
        line-height: 1;
    }
    ol, ul {
        list-style: none;
    }
    blockquote, q {
        quotes: none;
    }
    blockquote:before, blockquote:after,
    q:before, q:after {
        content: '';
        content: none;
    }
    table {
        border-collapse: collapse;
        border-spacing: 0;
    }
    
    /*----------------------------------------------------------------------*/
    /* End of CSS Reset */
    /*----------------------------------------------------------------------*/
    
    
    
    
    html, body {
        height: 100%;
        background-color: white;
        font-family: 'Work Sans', Raleway, sans-serif;
        }
    
    p {
        text-align: center;
        color: #333333
        }
    
    h1 {
        text-align: center;
        padding: 10px 0px;
        color: #1c76bc;
        font-size: 28px;
        font-weight: bold;
        margin-top: 20px;
        }
    
    details > summary {
        text-align: center;
        padding: 10px 0px;
        color: #1c76bc;
        font-size: 28px;
        font-weight: bold;
    }
    
    details > summary:focus {
        outline: 0px
    }
    
    h2 {
        text-align: center;
        padding: 10px 0px;
        color: #333333;
        font-size: 20px;
        font-weight: bold;
        }
    
    h3 {
        text-align: center;
        padding: 10px 0px;
        color: #333333;
        font-size: 16px;
        }
    
    table {
        text-align:center;
        border-collapse:collapse;
        margin:auto;
        /* line-height:10px; */
        border:1px solid black;
        }
    
    td {
        padding:10px;
        /* line-height: 120%; */
        font-size:14px;
        color: #333333;
        border:1px solid black;
        }
    
    th {
        padding:10px;
        line-height: 120%;
        font-size:14px;
        color: #333333;
        border:1px solid black;
        background-color: #cce0ff;
        font-weight: bold;
        }
    
    sup {
        vertical-align: super;
        font-size: smaller;
        }
    
    div {
        padding: 20px 0px;
        }
    
    .navbar {
        position: fixed;
        background-color: #cce0ff;
        margin: 0;
        border-bottom: 1px solid #1c76bc ;
        width: 100%;
        height: auto;
        padding: 5px;
        }
    
    .navbar ul {
        list-style: none;
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        }
    
    .navbar li a {
        padding: 5px 20px;
        line-height: 170%;
        text-align: center;
        text-decoration: none;
        font-size: 16px;
        color: rgba(51,51,51,1);
        }
    
    .navbar a:hover {
        background-color: #1c76bc;
        color: #FFFFFF;
        }
    
    #logo {
        text-align: center;
        padding-top: 100px;
        }
    
    #logo img{
        display: inline-block;
        width: 50%;
        height: auto;
        }
    
    #heading h1 {
        font-size: 60px;
        padding-top: 100px;
    }
    
    .details {
        padding: 10px;
        table-layout: fixed;
        width: 50%;
        text-align:center;
        border-collapse:collapse;
        }
    
    .details td {
        font-size: 20px;
        text-align:center;
        width: 50%;
        word-wrap: break-word;
        vertical-align: middle;
        }
    
    .tally {
        padding-bottom: 50px;
    }
    
    .tally_results {
        padding: 1px;
    }
    
    .tally_checks {
        padding: 1px;
    }
    
    .termination_warning {
        color: red;
        font-size: 32px;
    }
    
    #warnings, #duplicates, #comments, #fatal_errors, #error_log {
        text-align: center;
        }
    
    #warning_messages, #duplicate_messages, #comment_messages, #fatal_messages, #error_log_messages {
        display:inline-block;
        padding: 5px
        }
    
    #fatal_messages, p {
        text-align: left;
        padding: 5px;
        }
    
    #warning_messages, p {
        text-align: left;
        padding: 5px;
        }
    
    #duplicate_messages, p {
        text-align: left;
        padding: 5px;
        }
    
    #comment_messages p {
        text-align: left;
        padding: 5px;}
    
    #input_deck{
        text-align: center;
        margin: auto;
        }
    
    #input {
        display: inline-block;
        }
    
    #input p {
        white-space:pre;
        font-family: Courier,monospace;
        text-align:left;
        }
    
    #cerberus_credit p {
        text-align: center;
        padding: 5px 50px 5px 50px;
        line-height: 150%;
        }
    
    
    </style>
    ");
}