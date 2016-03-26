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
      data: data
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

function loadBtsf(dataBuf) {
  var records = readBtsfFile(dataBuf);
}

function init() {
  fetchArrayBuffer("btsf/mortality.btsf", loadBtsf);
}

init();
