@Library('jenkins-joylib@v1.0.6') _

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
            joyMattermostNotification(channel: 'jenkins')
        }
    }

}
