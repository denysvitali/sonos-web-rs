/*
 * Sonos Web UI gulpfile.js
 * Based on @Falconerd gist: https://gist.github.com/Falconerd/3acc15f93b6a023e43b9
 */

const watchify = require('watchify');
const browserify = require('browserify');
const gulp = require('gulp');
const plumber = require('gulp-plumber');
// const uglify = require('gulp-uglify');
const source = require('vinyl-source-stream');
const buffer = require('vinyl-buffer');
const gutil = require('gulp-util');
const sourcemaps = require('gulp-sourcemaps');
const assign = require('lodash.assign');
const del = require('del');
const less = require('gulp-less');
const rename = require('gulp-rename');
const jade = require('gulp-jade');
const lessChanged = require('gulp-less-changed');
const minify = require('gulp-minify');

var customOpts = {
    entries: ['./src/js/main.js'],
    debug: true,
    transform: [
        ['babelify', {
            presets: ['es2015'],
            ignore: ['./src/js/libs/**']
        }]
    ],
    ignore: ['./src/js/libs/**']
};
var opts = assign({}, watchify.args, customOpts);
var b = browserify(opts);
b.on('log', gutil.log);

/**
 * This task removes all files inside the 'public' directory.
 */
gulp.task('clean', function () {
    'use strict';
    del.sync('./public/**/*');
});

/**
 * This task will copy all files from libs into 'public/js/libs'.
 * If you want to process them, just add your code to this task.
 */
gulp.task('libs', ['clean'], function () {
    'use strict';
    return gulp.src(['./src/js/libs/**'])
        .pipe(plumber())
        .pipe(gulp.dest('./public/js/libs'));
});


/**
 * This task will minify all the js files and put them in
 * `/public/js`
 */
gulp.task('js', ['clean'], function () {
    'use strict';
    return gulp.src(['./src/js/*.js'])
        .pipe(plumber())
        .pipe(minify())
        .pipe(gulp.dest('./public/js/'));
});

/**
 * This task will copy all files from media into 'public/fonts'.
 * If you want to process them, just add your code to this task.
 */
gulp.task('media', ['js', 'libs'], function () {
    'use strict';
    return gulp.src(['./src/img/**'])
        .pipe(plumber())
        .pipe(gulp.dest('./public/img'));
});

/**
 * This task will copy all files from media into 'public/fonts'.
 * If you want to process them, just add your code to this task.
 */
gulp.task('fonts', ['media'], function () {
    'use strict';
    return gulp.src(['./src/fonts/**'])
        .pipe(plumber())
        .pipe(minify())
        .pipe(gulp.dest('./public/fonts'));
});

/**
 * This task will copy css files into 'public/css'
 * If you want to process it, just add your code to this task.
 */
gulp.task('css', ['fonts'], function () {
    'use strict';
    return gulp.src(['./src/css/**'])
        .pipe(plumber())
        .pipe(gulp.dest('./public/css'));
});

gulp.task('less', ['css'], function () {
    'use strict';
    console.log('Compiling less...');
    return gulp.src('./src/less/main.less')
        .pipe(plumber())
        .pipe(less())
        .pipe(gulp.dest('./public/css'));
});

/**
 * This task will render all the layouts to 'public/'.
 */
gulp.task('jade', ['less'], function () {
    'use strict';
    return gulp.src(['./src/jade/**'])
        .pipe(plumber())
        .pipe(jade())
        .pipe(gulp.dest('./public/'));
});

/**
 * This task will bundle all other js files and babelify them.
 * If you want to add other processing to the main js files, add your code here.
 */
gulp.task('bundle', ['jade'], function () {
    'use strict';
    return b.bundle()
        .on('error', function (err) {
            console.log(err.message);
            this.emit('end');
        })
        .pipe(source('./src/js/main.js'))
        .pipe(buffer())
        .pipe(sourcemaps.init({
            loadMaps: true
        }))
        .pipe(sourcemaps.write('./'))
        .pipe(rename('app.min.js'))
        .pipe(gulp.dest('./public/js'));
});

/**
 * This task starts watching the files inside 'src'. If a file is changed,
 * removed or added then it will run refresh task which will run the bundle task
 * and then refresh the page.
 *
 * For large projects, it may be beneficial to separate copying of libs and
 * media from bundling the source. This is especially true if you have large
 * amounts of media.
 */
gulp.task('watch', ['bundle'], function () {
    'use strict';
    var watcher = gulp.watch(['./src/**/*', './src/less/_include/*.less', './plugins/**/*.less'], []);
    watcher.on('change', function (event) {
        console.log('File ' + event.path + ' was ' + event.type + ', running tasks...');
    });
});

/**
 * This is the default task which chains the rest.
 */
gulp.task('default', ['watch']);
