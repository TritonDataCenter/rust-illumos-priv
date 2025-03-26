/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright 2025 MNX Cloud, Inc.
 */

@Library('jenkins-joylib@v1.0.8') _

pipeline {

    agent {
        // Currently set to the earliest Rust-included build/origin image
        // in use by release repos.
        label joyCommonLabels(image_ver: '19.4.0')
    }

    options {
        buildDiscarder(logRotator(numToKeepStr: '30'))
        timestamps()
    }

    stages {
        stage('check') {
            steps{
                sh('make check')
            }
        }
        stage('test') {
            steps{
                sh('make test')
            }
        }
    }

    post {
        always {
            joySlackNotifications()
        }
    }

}
