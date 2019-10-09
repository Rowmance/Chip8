import * as wasm from 'chip8';

console.log('hello', wasm.add(4, 4));
wasm.main();

const frameCallback = () => {
  if (!paused) {
    tickN(speed * 10);
  }
  window.requestAnimationFrame(frameCallback);
};
window.requestAnimationFrame(frameCallback);

function tick() {
  const tick = wasm.tick();
  // console.log(tick.op_code, tick.ptr);
  addOp(tick.op_code);
  tick.free();
}

function tickN(count) {
  while (count-- > 0) {
    tick();
  }
}

////////////// OP CODES
function opCodeData(opCode) {
  const nibbles = [
    (opCode & 0xF000) >> 12,
    (opCode & 0x0F00) >> 8,
    (opCode & 0x00F0) >> 4,
    (opCode & 0x000F),
  ];
  const kk = opCode & 0x00FF;
  const addr = opCode & 0x0FFF;
  return {
    name: opCodeName(nibbles),
    x: nibbles[1],
    y: nibbles[2],
    n: nibbles[3],
    kk: kk,
    addr: addr
  }
}

function opCodeName(nibbles) {
  switch (nibbles[0]) {
    case 0x00:
      return nibbles[3] === 0x00 ? 'Clear Screen' : 'Return';
    case 0x01:
      return 'Goto ADDR';
    case 0x02:
      return 'Call ADDR';
    case 0x03:
      return 'Skip If Vx == KK';
    case 0x04:
      return 'Skip If Vx != KK';
    case 0x05:
      return 'Skip If Vx == Vy';
    case 0x06:
      return 'Set Vx = KK';
    case 0x07:
      return 'Set Vx += KK';
    case 0x08:
      switch (nibbles[3]) {
        case 0x00:
          return 'Set Vx = Vy';
        case 0x01:
          return 'Set Vx = Vx | Vy';
        case 0x02:
          return 'Set Vx = Vx & Vy';
        case 0x03:
          return 'Set Vx = Vx ^ Vy';
        case 0x04:
          return 'Set Vx = Vx + Vy';
        case 0x05:
          return 'Set Vx = Vx - Vy';
        case 0x06:
          return 'Set Vx = Vx >> 1';
        case 0x07:
          return 'Set Vx = Vy - Vx';
        case 0x0E:
          return 'Set Vx = Vx << 1';
      }
    case 0x09:
      return 'Skip If Vx != Vy';
    case 0x0A:
      return 'Set I to ADDR';
    case 0x0B:
      return 'Jump to ADDR + V0';
    case 0x0C:
      return 'Set Vx = rand() & KK';
    case 0x0D:
      return 'Draw N at (Vx, Vy)';
    case 0x0E:
      switch (nibbles[3]) {
        case 0x0E:
          return 'Skip if KeyPressed(Vx)';
        case 0x01:
          return 'Skip if not KeyPressed(Vx)';
      }
    case 0x0F:
      switch (nibbles[2]) {
        case 0x00:
          switch (nibbles[3]) {
            case 0x07:
              return 'Set Vx = DelayTimer';
            case 0x0A:
              return 'Set Vx = Await KeyPress';
          }
        case 0x01:
          switch (nibbles[3]) {
            case 0x05:
              return 'Set DelayTimer = Vx';
            case 0x08:
              return 'Set SoundTimer = Vx';
            case 0x0E:
              return 'Set I = I + Vx'
          }
        case 0x02:
          return 'Set I = Sprite[vX]';
        case 0x03:
          return 'Convert Vx to Binary';
        case 0x05:
          return 'Store V0..X to Mem starting at I';
        case 0x06:
          return 'Load V0..X from Mem starting at I';
        default:
          return 'No Operation'
      }
  }
}

function toBase(number, base) {
  if (base === 16) {
    return `0x${number.toString(base).toUpperCase()}`;
  }
  return number.toString(base).toUpperCase();
}

function addOp(op) {
  const opCode = toBase(op, 16);
  const info = opCodeData(op);
  const newNode = document.createElement('tr');
  newNode.innerHTML = `<tr>
        <td>${opCode}</td>
        <td>${info.name}</td>
        <td>${toBase(info.x, 16)}</td>
        <td>${toBase(info.y, 16)}</td>
        <td>${toBase(info.n, 16)}</td>
        <td>${toBase(info.kk, 16)}</td>
        <td>${toBase(info.addr, 16)}</td>
      </tr>`;
  const element = document.getElementById("instruction-table");
  element.insertBefore(newNode, element.firstChild.nextSibling.nextSibling);
  if (element.childElementCount > 20) {
    element.removeChild(element.lastChild);
  }
}

////////////// PAUSE/STEP/RESUME BUTTONS
let paused = false;

function onPause() {
  paused = true;
}

document.getElementById('btn-pause').addEventListener('click', () => {
  onPause();
  document.getElementById('btn-pause').classList.add('button--disabled');
  document.getElementById('btn-resume').classList.remove('button--disabled');
  document.getElementById('btn-step').classList.remove('button--disabled');
});

function onResume() {
  paused = false;
}

document.getElementById('btn-resume').addEventListener('click', () => {
  onResume();
  document.getElementById('btn-pause').classList.remove('button--disabled');
  document.getElementById('btn-resume').classList.add('button--disabled');
  document.getElementById('btn-step').classList.add('button--disabled');
});

function onStep() {
  if (paused) {
    tick();
  }
}

document.getElementById('btn-step').addEventListener('click', () => {
  onStep();
});


////////////// SPEED BUTTONS
let speed = 1;

function setSpeed(newSpeed) {
  speed = newSpeed;
  document.getElementById('btn-speed0').classList.remove('button--selected');
  document.getElementById('btn-speed1').classList.remove('button--selected');
  document.getElementById('btn-speed2').classList.remove('button--selected');
  document.getElementById('btn-speed3').classList.remove('button--selected');
}

document.getElementById('btn-speed0').addEventListener('click', () => {
  setSpeed(0.5);
  document.getElementById('btn-speed0').classList.add('button--selected');
});
document.getElementById('btn-speed1').addEventListener('click', () => {
  setSpeed(1);
  document.getElementById('btn-speed1').classList.add('button--selected');
});
document.getElementById('btn-speed2').addEventListener('click', () => {
  setSpeed(2);
  document.getElementById('btn-speed2').classList.add('button--selected');
});
document.getElementById('btn-speed3').addEventListener('click', () => {
  setSpeed(3);
  document.getElementById('btn-speed3').classList.add('button--selected');
});