const { minify } = require('terser');
const fs = require('fs');
const path = require('path');

async function compressFile(inputPath, outputPath) {
    try {
        const code = fs.readFileSync(inputPath, 'utf8');
        const result = await minify(code, {
            compress: true,
            mangle: true,
            format: {
                comments: false
            }
        });

        if (result.error) {
            console.error('Error during compression:', result.error);
            process.exit(1);
        }

        fs.writeFileSync(outputPath, result.code);
        console.log(`Successfully compressed ${inputPath} to ${outputPath}`);
    } catch (error) {
        console.error('Error:', error.message);
        process.exit(1);
    }
}

// Get command line arguments
const args = process.argv.slice(2);
if (args.length !== 2) {
    console.error('Usage: node index.js <input-file> <output-file>');
    process.exit(1);
}

const [inputFile, outputFile] = args;
compressFile(inputFile, outputFile); 