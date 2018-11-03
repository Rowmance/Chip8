/*
 * Copyright (C) 2018 - present by OpenGamma Inc. and the OpenGamma group of companies
 *
 * Please see distribution for license.
 */

import { Injectable } from '@angular/core';
import { from } from 'rxjs/internal/observable/from';


@Injectable()
export class WasmService {

  mod: any;

  tick() {
    if (!this.mod) {
      return;
    }
    this.mod.tick()
  }

  load() {
    from(import('chip8'))
      .subscribe(x => {
        x.main();
        this.mod = x;
      });
  }

  constructor() {
    this.load();
  }

}
