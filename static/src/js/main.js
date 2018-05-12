function gRequest(url, cb) {
    if (window.XMLHttpRequest) { // Mozilla, Safari, IE7+ ...
        httpRequest = new XMLHttpRequest();
    } else if (window.ActiveXObject) { // IE 6 and older
        httpRequest = new ActiveXObject("Microsoft.XMLHTTP");
    }

    httpRequest.onreadystatechange = () => {
        if (httpRequest.readyState === 4 && httpRequest.status === 200) {
            cb(httpRequest);
        }
    };

    httpRequest.open('GET', url, true);
    httpRequest.send();
}

function track_info() {
    return new Promise((resolve,reject) => {
        gRequest('/api/0/track_info', (xhr) => {
            if (xhr.status == 200) {
                let json = JSON.parse(xhr.responseText);
                if (json.meta.success == true) {
                    resolve(json.track);
                } else {
                    reject(null);
                }
            }
        });
    });
}

let playbarprogress = document.getElementById('playbar-progress');
let running_time = document.getElementById('running-time');

let songbox = document.getElementById('song_box');
let songbox_songinfo_albumart = songbox.getElementsByClassName('album_cover')[0];
let songbox_songinfo = document.getElementById('sb_song_info');
let songbox_songinfo_title = songbox_songinfo.getElementsByClassName('title')[0];
let songbox_songinfo_artist = songbox_songinfo.getElementsByClassName('artist')[0];
let songbox_songinfo_album = songbox_songinfo.getElementsByClassName('album')[0];

let m_album_art = '';

function formatTime(time){
    let minutes = Math.floor(time/60);
    let seconds = time - minutes * 60;

    let minutes_ls = ('0' + minutes).substr(-2,2);
    let seconds_ls = ('0' + seconds).substr(-2, 2);

    return `${minutes_ls}:${seconds_ls}`;
}

function updateSongInfo(){
    track_info().then((json) => {
        songbox_songinfo_title.innerText = json.title;
        songbox_songinfo_artist.innerText = json.artist;
        songbox_songinfo_album.innerText = json.album;

        if(m_album_art != json.album_art){
            m_album_art = json.album_art;
            songbox_songinfo_albumart.src = json.album_art;
        }
        playbarprogress.style.width = json.running_time.secs / json.duration.secs * 100 + '%';
        running_time.innerText = `${formatTime(json.running_time.secs)} / ${formatTime(json.duration.secs)}`;

    }).catch((err) => {
        console.error(`Error! ${err}`)
    });
}

function updateLoop(){
    updateSongInfo();
    setTimeout('updateLoop()', 1000);
}

function init() {
    console.log('Init!');
    updateLoop();
}

init();