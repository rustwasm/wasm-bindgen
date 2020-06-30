export function initialize(lat, lon) {
    var myLatlng = new google.maps.LatLng(lat, lon);
    var myOptions = {
        zoom: 3,
        center: myLatlng,
        mapTypeId: google.maps.MapTypeId.ROADMAP
    }
    var map = new google.maps.Map(document.getElementById("map_canvas"), myOptions);

    var marker = new google.maps.Marker({
        position: myLatlng,
        title:"Hello World!"
    });

    marker.setMap(map);
}