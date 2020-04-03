var el = document.getElementById('parent');

function getImg() {
    var p = document.createElement('a');

    p.id = 'txt';
    var getJSON = function(url, callback) {
        var xhr = new XMLHttpRequest();
        xhr.open('GET', url, true);
        xhr.responseType = 'json';
        xhr.onload = function() {
            var status = xhr.status; 
        if (status === 200) {
            callback(null, xhr.response);
        } else {
            callback(status, xhr.response);
        }
    };
        xhr.send();
    };

    getJSON('http://localhost:8000/api/v1/images/', function(err, data) {
        var divdb = document.createElement('div');
        el.appendChild(divdb);
        for(var i=0; i<200; i++){
            var name = document.createElement('a');
            var img = document.createElement('img');
            img.width='100';
            var nmai = 'img:' + JSON.stringify(data.result[i].nameimg); 
            nmai = nmai.replace(/"/g, '');
            name.innerHTML = nmai;
            var imgs = JSON.stringify(data.result[i].img);
            imgs = imgs.replace(/"/g, '');
            img.src = imgs;
            divdb.appendChild(name);
            divdb.appendChild(img);
        }
    });
    el.appendChild(p);
}  

function preview() {
    var urlImg = document.getElementById('urlImg');
    var a = document.createElement('p');
    a.innerHTML = 'Введите имя файла';
    el.appendChild(a);
    var name = document.createElement('input');
    name.type = 'text';
    name.id = 'nameInp';
    el.appendChild(name);
    var img = document.createElement('img');
    img.id = 'output';
    img.height = '100';
    img.src = url.value;
    el.appendChild(img)
}
 function post() {
    var xhr = new XMLHttpRequest();
    xhr.onload = function() {
        var reader = new FileReader()
        reader.onloadend = function() {
            imgN = reader.result;
            var nameInp = document.getElementById('nameInp')         
            var pst = new XMLHttpRequest();
            pst.open('POST', 'http://localhost:8000/api/v1/images/', true);
            pst.setRequestHeader('Content-Type', 'application/json');
            pst.onreadystatechange = function() {
                if(pst.readyState === 4 && pst.status === 200) {
                    var json = JSON.parse(pst.responseText);
                    alert(json.result[0].nameimg + ", " + json.result[0].img);
                }
            }; 
            var data = JSON.stringify({"nameimg": nameInp.value, "img": imgN});
            pst.send(data);
        }
        reader.readAsDataURL(xhr.response);
    }
    xhr.open('GET', url.value);
    xhr.responseType = 'blob';
    xhr.send();
 }