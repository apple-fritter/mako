import 'zx/globals';

function replaceText(text) {
  // 替换所有「by @sorrycc」里的「@sorrycc」为链接
  const userRegex = /by (@([a-zA-z0-9\-_]+))/g;
  text = text.replace(userRegex, (match, mention, username) => {
    return `by [@${username}](https://github.com/${username})`;
  });

  // 替换所有「in #709」里的「#709」为链接
  const issueRegex = /in (#(\d+))/g;
  text = text.replace(issueRegex, (match, issue, number) => {
    return `in [#${number}](https://github.com/umijs/mako/pull/${number})`;
  });

  return text;
}

const cwd = process.cwd();
const changelogPath = path.join(cwd, 'CHANGELOG.md');
const content = fs.readFileSync(changelogPath, 'utf-8');
const newContent = replaceText(content);
fs.writeFileSync(changelogPath, newContent, 'utf-8');
console.log('CHANGELOG.md updated');