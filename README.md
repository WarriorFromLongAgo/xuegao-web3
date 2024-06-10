# xuegao-web3
采用分支的方式，手写框架，学习框架使用

# 创建项目目录
mkdir my-project
cd my-project

# 创建源码文件
mkdir src
touch src/main.ts src/main.test.ts src/cli.ts

# 创建package.json
yarn init

# 安装TypeScript, linter,  Jest
yarn add -D typescript @types/node ts-node
yarn add -D eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin
yarn add -D jest ts-jest @types/jest

# 获取.gitignore
curl -O https://raw.githubusercontent.com/metachris/typescript-boilerplate/master/.gitignore

# 也可以自己生成一个tsconfig.json
yarn global add typescript
C:\Users\65126\AppData\Local\Yarn\bin\tsc.cmd --init

# 获取tsconfig.json
curl -O https://raw.githubusercontent.com/metachris/typescript-boilerplate/master/tsconfig.json

# 获取.eslintrc.js
curl -O https://raw.githubusercontent.com/metachris/typescript-boilerplate/master/.eslintrc.js

# 获取jest.config.json, 可提供给ts-jest在不需要单独对typescript编译的情况下做测试
curl -O https://raw.githubusercontent.com/metachris/typescript-boilerplate/master/jest.config.js

# 初始化git并提交
git init
git add .
git commit -am "initial commit"

# 在package.json中扩展script字段：

{
"scripts": {
"cli": "ts-node src/cli.ts",
"test": "jest",
"lint": "tslint -c tslint.json 'src/**/*.ts'",
"build": "tsc -p tsconfig.json",
"clean": "rm -rf dist build",
"ts-node": "ts-node"
}
}

现在，已经可以在项目中执行 yarn cli, yarn test, yarn lint, yarn build, yarn ts-node <filename>。


通过Jest做测试
添加Jest测试代码，例如：

import {greet} from './main'

test('the data is peanut butter', () => {
expect(1).toBe(1)
});

test('greeting', () => {
expect(greet('Foo')).toBe('Hello Foo')
});
可以通过yarn test执行测试，而不需要再单独的编译步骤。

查看 详细Jest文档
查看其他测试工具：ava, uvu, tape

esbuild
esbuild 是一个执行贼快的JavaScript打包工具，它主打性能这块，也可用于编译TypeScript代码， 为不同运行环境(浏览器/Node)进行打包。esbuild现在还比较年轻而且还在积极开发中，可以到 esbuild on GitHub 详细了解。

为什么要用esbuild而不直接用tsc？是因为esbuild不能很好的为浏览器环境做打包(所以常借助额外的打包工具，例如webpack,parcel,rollup)，而且tsc特别慢。

安装esbuild:

yarn add -D esbuild


Node.js环境打包
除了tsc，还可以使用esbuild为目标环境为Node.js做打包，例如：

# 打包
yarn esbuild src/cli.ts --bundle --platform=node --outfile=dist/esbuild/cli.js

# 打包&minify&sourcemap
yarn esbuild src/cli.ts --bundle --platform=node --minify --sourcemap=external --outfile=dist/esbuild/cli.js

# 执行&输出
node dist/esbuild/cli.js
esbuild的更多参数可以到esbuild的文档中查看。


注意：

可通过--watch参数监听文件发生变化后重新构建
目前esbuild还不支持生成d.ts声明文件(issue)，如果需要生成声明文件还是需要tsc。
查看示例，该示例中包含了有关esbuild的命令。
构建兼容浏览器环境的模块
可以通过esbuild, webpack, parcel等打包工具打包成兼容浏览器环境的模块。

本文示例使用esbuild:

# 打包
yarn esbuild src/browser.ts --bundle --outfile=dist/esbuild/browser.js

# 并且 minify&sourcemap
yarn esbuild src/browser.ts --bundle --minify --sourcemap=external --outfile=dist/esbuild/browser.js
生成的browser.ts，可用于在浏览器中加载并执行。

访问Dom属性
在浏览器环境中代码可以访问window或document对象，以及可能需要挂载一些值到window对象上。

在tsconfig.json中，添加Dom相关的预定义声明：

{
"lib": [
"ES6",
"DOM"
]
}
添加src/browser.ts文件，用于浏览器环境打包的入口。文件中代码在window添加了新的属性。

// file: src/browser.ts
import {greet} from './main'

(window as any).greet = greet
执行打包：

yarn esbuild src/browser.ts --bundle --outfile=dist/esbuild/browser.js





