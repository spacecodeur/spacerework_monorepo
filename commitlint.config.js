module.exports = {
    extends: ['@commitlint/config-conventional'],
    rules: {
        'type-enum': [
            2,
            'always',
            [
                'feature',   // New feature
                'fix',       // Bug fix
                'chore',     // Maintenance or tooling update
                'docs',      // Documentation changes
                'test',      // Adding/updating tests
                'refactor',  // Code improvement without behavior change
                'poc',       // Proof of Concept (experimental implementation)
                'perf',      // Performance improvements
                'ci'         // CI/CD updates
            ]
        ],
        'subject-case': [
            2, 
            'always', 
            'lower-case'
        ],
    },
};