window.onresize = function(event) {
    $("#body").css("top", ($("#navbar").height()+16)+"px");
    $(".w-100").css("width", ($("#navbar").width()-8-61.4)+"px");
    $(".width-90-12").css("width", (($("#search").width()*0.9)-12)+"px");
    $(".width-10-4").css("width", (($("#search").width()*0.1)-4)+"px");
    $("#grid").css("grid-template-columns", "repeat("+Math.floor($("#body").width()/270)+",minmax(0,1fr))");
};

$(document).ready(function() {
    $("#body").css("top", ($("#navbar").height()+16)+"px");
    $(".w-100").css("width", ($("#navbar").width()-8-61.4)+"px");
    $(".width-90-12").css("width", (($("#search").width()*0.9)-12)+"px");
    $(".width-10-4").css("width", (($("#search").width()*0.1)-4)+"px");
    $("#grid").css("grid-template-columns", "repeat("+Math.floor($("#body").width()/270)+",minmax(0,1fr))");
});