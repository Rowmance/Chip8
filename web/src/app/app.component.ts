import { Component } from '@angular/core';
import { WasmService } from './wasm-service';

@Component({
  selector: 'c8-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'www';

  constructor(wasm: WasmService) {
    wasm.greet('hello');
  }
}
