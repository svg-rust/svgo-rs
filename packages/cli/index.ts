#!/usr/bin/env node

import colors from 'picocolors';
import { program } from 'commander';
import makeProgram from './coa';

makeProgram(program);
program.parseAsync(process.argv).catch(error => {
  console.error(colors.red(error.stack));
  process.exit(1);
});