var movies = [];

window.onresize = function(event) {
    resize_all();
};

$(document).ready(function() {
    resize_all();
    if ($("#grid").length > 0) {
        $.get("config.json", function(data) {
            for(var i in data) {
                $.get(`lookup/${i}`, function(data) {
                    var data = JSON.parse(data);
                    movies[data.tt_url.replace(/\//g, "").split("titlett")[1].trim()] = data;
                    console.log(movies[data.tt_url.replace(/\//g, "").split("titlett")[1].trim()]);
                    $("#grid").html($("#grid").html()+`<div class="w-64 h-min-[75%] h-96 bg-gray-600 rounded-lg mt-4 overflow-clip cursor-pointer shadow-2xl hover:scale-110 transition-all duration-200" onclick="open_id(${data.tt_url.replace(/\//g, "").split("titlett")[1].trim()})"><img class="h-80 w-full object-cover rounded-t-lg" src="${data.poster}" alt="${data.title}"><p class="text-center align-middle">${data.title}</p></div>`);
                    $("#loader").css("display", "none");
                })
            }
        });
    }
});

function resize_all() {
    $("#body").css("top", ($("#navbar").height()+16)+"px");
    $(".w-100").css("width", ($("#navbar").width()-8-61.4)+"px");
    $(".width-90-12").css("width", ($("#search").width()-44)+"px");
    $("#grid").css("grid-template-columns", "repeat("+Math.floor($("#body").width()/300)+",minmax(0,1fr))");
    $("#loader").css("left", (($("#body").width()/2)-60)+"px");
    $("#loader").css("top", (($("#real_body").height()/2)-60-$("#navbar").height())+"px");
    $(".h-screen-navbar").css("height", ($(window).height()-$("#navbar").height()-24)+"px");
    $(".h-screen-navbar-24").css("height", ($(window).height()-$("#navbar").height()-16-24-8)+"px");
    $(".h-screen-navbar-24").css("width", (($(".h-screen-navbar").width()*0.75)-24)+"px");
    $("#overlay-image").css("height", ($(".overlay").height()-60)+"px");
    $("#loader1").css("left", (($(".overlay").width()/2)-60)+"px");
    $("#loader1").css("top", (($(".overlay").height()/2)-60)+"px");
}

function open_id(id) {
    $(".overlay").css("right", "1%");
    console.log(movies[id]);
    var data = movies[id];
    $("#overlay-image").attr("src", data.poster);
    $("#play").attr("onclick", "location = "+data.tt_url.replace(/\//g, "").split("titlett")[1]);
    $("#title").html(data.title);
    $("#rating").html(data.UserRating.numeric_rating_only);
    $("#description").html(data.short_imdb_description);
    $("#contentRating").html(data.contentRating);
    $("#length").html(data.duration);
    $("#date").html(data.jsonnnob.datePublished.split("-")[0]);
    var genres = [];
    for(var i in data.genres) {
        genres.push(data.genres[i].split("#")[1].replace(/_/g, " "));
    }
    $("#genres").html(genres.join(", "));
    $("#loader1-sheet").css("opacity", "0.0");
    setTimeout(function (){
        $("#loader1-sheet").css("display", "none");
    }, 500);
}

function close_id() {
    $(".overlay").css("right", "101%");
    setTimeout(function (){
        $("#loader1-sheet").css("opacity", "1.0");
        $("#loader1-sheet").css("display", "block");
    }, 1000);
}