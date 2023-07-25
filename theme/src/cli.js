#!/usr/bin/env node

const baseConfig = require('../config.json');
const { program } = require('commander');
const path = require('path');
const {readFileSync} = require('fs');
const _ = require('lodash');
const chokidar = require('chokidar');

const tokensPath = path.join(__dirname, '..', 'tokens');

program
  .command('build')
  .option('-f, --file <file>')
  .option('-p, --platforms <platforms...>')
  .option('-w, --watch')
  .description('Build the theme')
  .action((args, a) => {
    let config = _.merge(baseConfig, {
      source: [path.join(tokensPath, '**', '*.json')],
    });
    const optConfigPath = args.file;
    if (optConfigPath) {
      const configPath = path.join(process.cwd(), optConfigPath);
      const data = readFileSync(configPath, 'utf-8');
      config = _.merge(config, JSON.parse(data));
    }
    const build = () => require('./build')({
      config, 
      platforms: args.platforms,
    });
    if (args.watch) {
      return chokidar.watch(tokensPath)
        .on('ready', () => build())
        .on('change', () => build()) 
    }
    return build();
  })

program.parse(process.argv);
