window.onresize = function(event) {
    $("#body").css("top", ($("#navbar").height()+16)+"px");
    $(".w-100").css("width", ($("#navbar").width()-8-61.4)+"px");
    $(".width-90-12").css("width", ($("#search").width()-44)+"px");
    $("#grid").css("grid-template-columns", "repeat("+Math.floor($("#body").width()/270)+",minmax(0,1fr))");
    $("#loader").css("left", (($("#body").width()/2)-60)+"px");
    $("#loader").css("top", (($("#real_body").height()/2)-60-$("#navbar").height())+"px");
};

$(document).ready(function() {
    $("#body").css("top", ($("#navbar").height()+16)+"px");
    $(".w-100").css("width", ($("#navbar").width()-8-61.4)+"px");
    $(".width-90-12").css("width", ($("#search").width()-44)+"px");
    $("#grid").css("grid-template-columns", "repeat("+Math.floor($("#body").width()/270)+",minmax(0,1fr))");
    $("#loader").css("left", (($("#body").width()/2)-60)+"px");
    $("#loader").css("top", (($("#real_body").height()/2)-60-$("#navbar").height())+"px");
});