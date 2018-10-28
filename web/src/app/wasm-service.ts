/*
 * Copyright (C) 2018 - present by OpenGamma Inc. and the OpenGamma group of companies
 *
 * Please see distribution for license.
 */

import { Injectable } from '@angular/core';
import { from } from 'rxjs/internal/observable/from';


@Injectable()
export class WasmService {

  start(mymod: any) {
    console.log('All modules loaded');
    mymod.greet('hello');
  }

  load() {
    from(import('chip8'))
      .subscribe(x => this.start(x));
  }

  constructor() {
    this.load();
  }

  greet(_: string) {
    this.load();
  }

}
