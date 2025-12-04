/** @type {import("@commitlint/types").UserConfig} */
export default {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'type-enum': [
      1,
      'always',
      [
        'feat',
        'fix',
        'docs',
        'style',
        'refactor',
        'perf',
        'test',
        'ci',
        'chore',
        'revert',
        'build',
        'workflow',
        'mod',
        'wip',
        'types',
        'release',
      ],
    ],
    'subject-full-stop': [0, 'never'],
    'subject-case': [0, 'never'],
  },
};
