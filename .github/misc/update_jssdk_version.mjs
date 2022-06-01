import { readdirSync, readFileSync, writeFileSync } from 'fs';
import { join } from 'path';

function update_version(path, version) {
    let data = readFileSync(join(path, 'package.json'), { encoding: 'utf8' });
    let json = JSON.parse(data);
    json.version = version;
    let new_data = JSON.stringify(json, null, 2);
    writeFileSync(join(path, 'package.json'), new_data, { encoding: 'utf8' });
}

let version = process.argv.splice(2)[0];

update_version('./nodejs', version);

let dirs = readdirSync('./nodejs/npm');
for (let name of dirs) {
    update_version(join('./nodejs/npm', name), version);
}
