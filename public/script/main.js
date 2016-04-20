var normalizeYAxis = false;

// Requested data
var reqFilter = "";
var reqCorrelationQuery = null;

// Data currently being displayed
var curRecords = null;
var curOverlay = null;

var nextSeqNum = 1;
var displayingSeqNum = 0;


// http://stackoverflow.com/questions/149055/how-can-i-format-numbers-as-money-in-javascript
Number.prototype.formatNice = function(c, d, t){
  var n = this,
      c = isNaN(c = Math.abs(c)) ? 2 : c,
      d = d == undefined ? "." : d,
      t = t == undefined ? "," : t,
      s = n < 0 ? "-" : "",
      i = parseInt(n = Math.abs(+n || 0).toFixed(c)) + "",
      j = (j = i.length) > 3 ? j % 3 : 0;
     return s + (j ? i.substr(0, j) + t : "") + i.substr(j).replace(/(\d{3})(?=\d)/g, "$1" + t) + (c ? d + Math.abs(n - i).toFixed(c).slice(2) : "");
};


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

  return {size: H+L+N*8, record: {name: str, data: data, corr: C}};
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
  dv.setUint32(16+4*1, nameBuf.length, true);
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

function drawGraphLine(ctx,w,h,minT,maxT,data,trace) {
  // TODO: find start and end index of overlap and use that to optimize maxV, minV and iteration
  var maxV = _.max(data, function(p) {
    if(p.t < minT || p.t > maxT) return -Infinity;
    return p.v;
  }).v;
  var minV = _.min(data, function(p) {
    if(p.t < minT || p.t > maxT) return Infinity;
    return p.v;
  }).v;

  ctx.lineWidth = 1.0;
  ctx.beginPath();
  var drawnFirst = false;
  // TODO: don't render way more points than there are horizontal pixels in the graph
  for(var i = 0; i < data.length; i++) {
    if((((i+1) < data.length) && data[i].t < minT) || ((i-1) > 0 && data[i-1].t > maxT)) continue;
    var x = (data[i].t-minT)/(maxT-minT)*w;
    var yFrac;
    if(normalizeYAxis) {
      yFrac = (data[i].v-minV)/(maxV-minV);
    } else {
      yFrac = (data[i].v)/(maxV); // TODO: account for possible presence of negative numbers
    }
    var y = yFrac*(h-5)+2;
    if(drawnFirst === false) {
      ctx.moveTo(x,h-y);
      drawnFirst = true;
    }
    ctx.lineTo(x,h-y);
  }
  ctx.stroke();

  if(trace !== null) {
    var traceT = trace.x/w*(maxT-minT)+minT;
    var closestPt = _.min(data, function(p) { return Math.abs(p.t - traceT); });

    var x = Math.max(0, Math.min(w, (closestPt.t-minT)/(maxT-minT)*w));
    ctx.strokeStyle = "#EF5350";
    ctx.lineWidth = 2.0;
    ctx.beginPath();
    ctx.moveTo(x,h);
    ctx.lineTo(x,0);
    ctx.stroke();

    ctx.fillStyle = "#000";
    var textX = x+8;
    var date = new Date(closestPt.t*1000);
    var dateText = date.getFullYear() + "/" + (date.getMonth()+1);
    var valText = (closestPt.v).formatNice(2);

    var dateStyle = "10px sans-serif";
    ctx.font = dateStyle;
    var textW = ctx.measureText(dateText).width;

    ctx.font = "15px sans-serif";
    textW = Math.max(textW, ctx.measureText(valText).width);
    if(textX+textW > w) {
      textX = x - textW - 8;
    }
    ctx.fillText(valText, textX, trace.y);
    ctx.fillStyle = "#78909C"
    ctx.font = dateStyle;
    ctx.fillText(dateText, textX, trace.y-15);
  }
}

function drawGraph(canvasEl, data, trace) {
  var ctx = canvasEl.getContext("2d");
  ctx.fillStyle = "white";
  ctx.fillRect(0,0,canvasEl.width,canvasEl.height);

  var maxT = _.max(data, function(p) { return p.t; }).t;
  var minT = _.min(data, function(p) { return p.t; }).t;
  if(curOverlay !== null) {
    maxT = Math.min(maxT,_.max(curOverlay, function(p) { return p.t; }).t);
    minT = Math.max(minT,_.min(curOverlay, function(p) { return p.t; }).t);
    ctx.strokeStyle = "grey";
    drawGraphLine(ctx,canvasEl.width,canvasEl.height,minT,maxT, curOverlay, null);
  }
  ctx.strokeStyle = "#2196F3";
  drawGraphLine(ctx,canvasEl.width,canvasEl.height,minT,maxT, data, trace);
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

    var canvasEl = document.getElementById("canv-"+i);
    drawGraph(canvasEl, records[i].data, null);
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

      (function(){
        var curI = (numPresent+i);
        var curCanvas = canvas;
        canvas.addEventListener('mousemove', function(evt) {
          drawGraph(curCanvas, curRecords[curI].data, {x: evt.offsetX, y: evt.offsetY});
        }, false);
        canvas.addEventListener('mouseout', function(evt) {
          drawGraph(curCanvas, curRecords[curI].data, null);
        }, false);
      })();

      graphsDiv.appendChild(graphDiv);
    }
  } else if(delta < 0) {
    for(var i = 0; i < -delta; i++) {
      graphsDiv.children[numPresent-1-i].remove();
    }
  }
}

function redisplay() {
  normalizeYAxis = !document.getElementById("zeroYAxis").checked;
  if(reqCorrelationQuery !== null) {
    document.getElementById("clearCorrButton").style.display = "inline";
  } else {
    document.getElementById("clearCorrButton").style.display = "none";
  }
  displayRecords(curRecords, 100);
}

// ==== Changing requested data

function findCorrelations(record) {
  var dataBuf = serializeBtsfRecord(record);
  reqCorrelationQuery = dataBuf;
  curOverlay = record.data;

  reqFilter = "";
  document.getElementById('filter-box').value = "";
  document.getElementById('zeroYAxis').checked = false;

  updateFromServer();
}

function filterGraphs() {
  var filter = document.getElementById("filter-box").value;
  if(filter === reqFilter) return;
  reqFilter = filter;
  updateFromServer();
}

function clearCorr() {
  reqCorrelationQuery = null;
  curOverlay = null;
  updateFromServer();
}

// ==== Fetching requested data

function handleNewData(oEvent, seqNum) {
  // ignore out of order responses
  if(seqNum <= displayingSeqNum) return;
  displayingSeqNum = seqNum;

  var arrayBuffer = oEvent.target.response; // Note: not oReq.responseText
  if (arrayBuffer) {
    curRecords = readBtsfFile(arrayBuffer);
    redisplay();
  } else {
    console.log("Couldn't fetch file " + url);
  }
}

function fetchData(filter, corrBuffer) {
  var thisRequest = nextSeqNum;
  nextSeqNum += 1;

  var xhr = new XMLHttpRequest();
  var endpoint = (corrBuffer !== null) ? "/find?" : "/raw?";
  xhr.open("POST", endpoint+encodeURIComponent(filter), true);
  xhr.responseType = "arraybuffer";
  xhr.onload = function(oEvent) {
    handleNewData(oEvent, thisRequest);
  }

  if(corrBuffer !== null) {
    xhr.send(new DataView(corrBuffer));
  } else {
    xhr.send(null);
  }
}

function updateFromServer() {
  fetchData(reqFilter, reqCorrelationQuery);
}

function init() {
  updateFromServer();
  document.getElementById("filter-box").focus();
}
