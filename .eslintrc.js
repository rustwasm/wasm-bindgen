module.exports = { 
    env: { 
        es6: true, 
        browser: true, 
        commonjs: true, 
        node: true 
    }, 
    extends: 'eslint:recommended', 
    parserOptions: { 
        sourceType: 'module' 
    }, 
    rules: { 
        indent: ['error', 4], 
        'linebreak-style': [
            'error', 
            'unix'
        ], 
        quotes: [
            'error', 
            'single'
        ], 
        semi: [
            'error', 
            'always'
        ], 
        'no-console': 0, 
        'no-undef': 
        'warn', 
        'no-unused-vars': 'warn' 
    } 
};
