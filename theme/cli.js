#!/usr/bin/env node

const baseConfig = require('./config.json');
const { program } = require('commander');
const path = require('path');
const {readFile} = require('fs');
const _ = require('lodash');
const nodemon = require('nodemon');

program
  .command('build')
  .option('-f, --file <file>')
  .description('Build the theme')
  .action(function() {
    const configPath = path.join(process.cwd(), this.opts().file);
    readFile(configPath, 'utf-8', (err, data) => {
      if (err) throw err;
      const config = JSON.parse(data);
      require('./build')({
        config: _.merge(baseConfig, config, {
          source: [path.join(__dirname, 'tokens', '**', '*.json')],
        })
      });
    });
  })
  
program
  .command('watch')
  .option('-f, --file <file>')
  .description('Build the theme')
  .action(function() {
    nodemon({
      watch: [path.join(__dirname, 'tokens')],
      exec: `${path.join(__dirname, 'cli.js')} build -f ${this.opts().file}`,
      ext: 'json',
    });
  })

program.parse(process.argv);
