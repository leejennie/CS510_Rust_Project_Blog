Jennie Lee
Food Blog

This project is a recipe blog where people can post about different recipes. The goal of this blog is that users can make an account and post various recipes that they have such that others can use it. Visitors of the site should be able to navigate through to find different kinds of recipes that they are looking for, such as recipes for different italian food. They are not obligated to make an account if they choose not to, but they are free to look through and find different recipes. 

For building and running the project, Trunk will need to be installed. Here are the following instructions for it. 

Open terminal or console (or whatever you use to run command line programs)

clone repo

cd into the root directory of repo

Install Trunk:

    cargo install trunk wasm-bindgen-cli

    rustup target add wasm32-unknown-unknown

build and run the program using:
    
    trunk serve

For more documentation or if you run into issues, plase refer to these two websites that have documentation on trunk:
    
    https://github.com/thedodd/trunk

    https://github.com/yewstack/yew-trunk-minimal-template
    
After running trunk serve, it should compile and work, popping up a link that looks like:

    http://127.0.0.1:8080/

That will deploy the site locally.

Shut down: 

    ctrl-C 

Given the time and finally getting the code working, I did not have time to write test code that specifically test what I wrote, such as routing errors. I did not have time to implement the backend portion, so testing for that would include making sure that fields in the login and sign in pages are filled in correctly, etc. However, trunk itself has some test code that are included when building and running the program and that looks like it passed those tests since no errors were thrown.

An example illustrating the operation of my code is that say we start running the program. Initally, the homepage should be the first thing seen with the welcome message. Then, by clicking on the tabs in the navigation bar, the routing works to switch the pages from one to another.

I was able to get the skeleton up and running with trunk and once I was able to, it was easier to put together the different pages following examples from https://github.com/jetli/rust-yew-realworld-example-app/tree/master/crates/conduit-wasm and https://github.com/ahmad2smile/portfolio. I wrote the .css and the html sections in the different files because of previous coding experience with html and css coding. I was not able to get the site up and running on github pages due to errors, but I was able to get it running locally. I am not satisfied with the results due becasue the website is lacking in many aspects that I could have worked on if I had more time. I found that I was limited on time because when I intially started the project, I could not find a good template to start the project with since many of them required me to intall many different packages and all sorts of things. But working on the project the past few days, trunk was new to the YEw page and I just restarted the project and was able to get something up and running and some of the site styled. I would liek to add more pages and get more of the site up and running in the future. In addition, I would like to implement the backend portion to make the website more complete. It would also be nice to collaborate with others on this project. Just me alone wokring on doing a project with a backend and frontend portion would take lots of time alone. Also, I would liek to read up more on Rust and how it works with web applications since there are so many different web libraries that could be used.

The license can be found in LICENSE file in the root directory and it is the MIT license.

