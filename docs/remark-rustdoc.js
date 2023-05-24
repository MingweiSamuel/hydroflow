const visit = require('unist-util-visit');

module.exports = function rustdocLinks() {
    return (tree) => {
        visit(tree, 'text', (node, index, parent) => {
            if (node.value.includes('totally')) {
                require('fs').writeFileSync('./test.json', JSON.stringify({ node, index, parent }, null, 2));
            }
        });
    };
};
