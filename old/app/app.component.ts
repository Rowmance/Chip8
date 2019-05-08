// import { Component } from '@angular/core';
// import { WasmService } from './wasm-service';
//
// @Component({
//   selector: 'c8-root',
//   templateUrl: './app.component.html',
//   styleUrls: ['./styles.scss']
// })
// export class AppComponent {
//   paused = false;
//   speed = 1;
//
//   constructor(private wasm: WasmService) {
//     console.log('All modules loaded');
//     const frame = () => {
//       if (this.paused) {
//         window.requestAnimationFrame(frame);
//         return;
//       }
//       this.tickN(this.speed * 10);
//       window.requestAnimationFrame(frame)
//     };
//     window.requestAnimationFrame(frame);
//   }
//
//   tickN(count: number) {
//     while (count-- > 0) {
//       this.wasm.tick();
//     }
//   }
//
//   onPause() {
//     this.paused = true;
//   }
//
//   onResume() {
//     this.paused = false;
//   }
//
//   onStep() {
//     if (this.paused) {
//       this.wasm.tick();
//     }
//   }
//
//   setSpeed(speed: number) {
//     this.speed = speed;
//   }
// }
