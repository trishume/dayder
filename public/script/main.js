var allRecords = null;
var normalizeYAxis = false;
var curOverlay = null;

function fetchArrayBuffer(url, callback) {
  var oReq = new XMLHttpRequest();
  oReq.open("GET", url, true);
  oReq.responseType = "arraybuffer";

  oReq.onload = function (oEvent) {
    var arrayBuffer = oReq.response; // Note: not oReq.responseText
    if (arrayBuffer) {
      callback(arrayBuffer);
    } else {
      console.log("Couldn't fetch file " + url);
    }
  };

  oReq.send(null);
}

function readBtsfRecord(dataBuf, offset, H) {
  var dv = new DataView(dataBuf);
  var N = dv.getUint32(offset+4*0, true);
  var L = dv.getUint32(offset+4*1, true);

  var C = null;
  if(H > 8) {
    C = dv.getFloat32(offset+4*2, true);
  }

  var decoder = new TextDecoder();
  var strView = new DataView(dataBuf, offset+H, L);
  var str = decoder.decode(strView);

  var data = [];
  var datOffset = offset+H+L;
  for(var i = 0; i < N; i++) {
    var T = dv.getInt32(datOffset+i*8, true);
    var D = dv.getFloat32(datOffset+i*8+4, true);
    data.push({t: T, v: D});
  }

  return {
    size: H+L+N*8,
    record: {
      name: str,
      data: data,
      corr: C
    }
  };
}

function readBtsfFile(dataBuf) {
  var dv = new DataView(dataBuf);
  var V = dv.getUint32(0, true);
  var F = dv.getUint32(4*1, true);
  var H = dv.getUint32(4*2, true);
  var R = dv.getUint32(4*3, true);
  console.log("loaded file",[V,F,H,R]);

  var offset = F;
  var records = [];
  for(var i = 0; i < R; i++) {
    var res = readBtsfRecord(dataBuf, offset, H);
    offset += res.size;
    records.push(res.record);
  }
  return records;
}

function serializeBtsfRecord(record) {
  var encoder = new TextEncoder("utf-8");
  var nameBuf = encoder.encode(record.name);

  var length = 4*4+4*2+nameBuf.length+record.data.length*8;
  var dataBuf = new ArrayBuffer(length);
  var dv = new DataView(dataBuf);

  // header
  dv.setUint32(4*0, 1, true);
  dv.setUint32(4*1, 4*4, true);
  dv.setUint32(4*2, 4*2, true);
  dv.setUint32(4*3, 1, true);

  // record
  dv.setUint32(16+4*0, record.data.length, true);
  dv.setUint32(16+4*1, record.name.length, true);
  for (var i = 0; i < nameBuf.length; i++) {
    dv.setUint8(6*4+i, nameBuf[i]);
  }
  for (var i = 0; i < record.data.length; i++) {
    dv.setInt32(6*4+nameBuf.length+i*8, record.data[i].t, true);
    dv.setFloat32(6*4+nameBuf.length+i*8+4, record.data[i].v, true);
  }

  return dataBuf;
}

function maybeTrim(name, len) {
  if(name.length > len-3) {
    return name.slice(0,len-3)+"...";
  } else {
    return name;
  }
}

function drawGraphLine(ctx,w,h, data) {
  var maxV = _.max(data, function(p) { return p.v; }).v;
  var minV = _.min(data, function(p) { return p.v; }).v;
  var maxT = _.max(data, function(p) { return p.t; }).t;
  var minT = _.min(data, function(p) { return p.t; }).t;

  ctx.beginPath();
  ctx.moveTo(0,h);
  for(var i = 0; i < data.length; i++) {
    var x = (data[i].t-minT)/(maxT-minT)*w;
    var yFrac;
    if(normalizeYAxis) {
      yFrac = (data[i].v-minV)/(maxV-minV);
    } else {
      yFrac = (data[i].v)/(maxV);
    }
    var y = yFrac*(h-5)+2;
    if(i == 0) ctx.moveTo(x,h-y);
    ctx.lineTo(x,h-y);
  }
  ctx.stroke();
}

function drawGraph(graphNum, data) {
  var canvasEl = document.getElementById("canv-"+graphNum);
  var ctx = canvasEl.getContext("2d");

  ctx.fillStyle = "white";
  ctx.fillRect(0,0,canvasEl.width,canvasEl.height);

  if(curOverlay !== null) {
    ctx.strokeStyle = "grey";
    drawGraphLine(ctx,canvasEl.width,canvasEl.height, curOverlay);
  }
  ctx.strokeStyle = "#2196F3";
  drawGraphLine(ctx,canvasEl.width,canvasEl.height, data);
}

function displayRecords(records, maxRecords) {
  var numToDisplay = Math.min(maxRecords, records.length);
  setNumberOfGraphs(numToDisplay);

  for(var i = 0; i < numToDisplay; i++) {
    var label = document.getElementById("label-"+i);
    label.innerText = maybeTrim(records[i].name,60);
    label.title = records[i].name;

    var corrEl = document.getElementById("corr-"+i);
    if(records[i].corr !== null) {
      corrEl.style.display = "initial";
      corrEl.innerText = "r = " + records[i].corr.toFixed(3);
    } else {
      corrEl.style.display = "none";
    }

    var link = document.getElementById("btn-"+i);
    // needed because JS closures interact weirdly with loops
    (function(){
      var record = records[i];
      link.onclick = function() {
        findCorrelations(record);
      }
    })();

    drawGraph(i, records[i].data);
  }
}

// sets up the DOM with the right number of graph boxes
function setNumberOfGraphs(n) {
  var graphsDiv = document.getElementById("graphs");
  var numPresent = graphsDiv.children.length;
  var delta = n - numPresent;

  if(delta > 0) {
    for(var i = 0; i < delta; i++) {
      var graphDiv = document.createElement("div");
      graphDiv.className = "graph";

      var label = document.createElement("h4");
      label.id = "label-"+(numPresent+i);
      graphDiv.appendChild(label);

      var canvas = document.createElement("canvas");
      canvas.id = "canv-"+(numPresent+i);
      canvas.height = 160;
      canvas.width = 300;
      graphDiv.appendChild(canvas);

      var correlate = document.createElement("img");
      correlate.id = "btn-"+(numPresent+i);
      correlate.src = "graph-icon.svg";
      graphDiv.appendChild(correlate);

      var correlation = document.createElement("span");
      correlation.innerText = "r = 0.8";
      correlation.className = "correlation"
      correlation.id = "corr-"+(numPresent+i);
      graphDiv.appendChild(correlation);

      graphsDiv.appendChild(graphDiv);
    }
  } else if(delta < 0) {
    for(var i = 0; i < -delta; i++) {
      graphsDiv.children[numPresent-1-i].remove();
    }
  }
}

function filterGraphs() {
  var query = document.getElementById("filter-box").value;
  normalizeYAxis = !document.getElementById("zeroYAxis").checked;
  var records = _.filter(allRecords, function(r) {
    return r.name.includes(query);
  });
  displayRecords(records, 100);
}

function findCorrelations(record) {
  var dataBuf = serializeBtsfRecord(record);
  var xhr = new XMLHttpRequest;
  xhr.open("POST", "/find", true);
  xhr.responseType = "arraybuffer";
  xhr.onload = function (oEvent) {
    var arrayBuffer = xhr.response; // Note: not oReq.responseText
    if (arrayBuffer) {
      allRecords = readBtsfFile(arrayBuffer);

      curOverlay = record.data;
      document.getElementById('filter-box').value = "";
      document.getElementById('zeroYAxis').checked = false;

      filterGraphs();
    } else {
      console.log("Couldn't fetch file " + url);
    }
  };
  xhr.send(new DataView(dataBuf));
}

function loadBtsf(dataBuf) {
  allRecords = readBtsfFile(dataBuf);
  filterGraphs();
}

function init() {
  fetchArrayBuffer("btsf/mortality.btsf", loadBtsf);
  document.getElementById("filter-box").focus();
}
