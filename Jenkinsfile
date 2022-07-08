import hudson.tasks.test.AbstractTestResultAction
import hudson.model.Actionable
import hudson.tasks.junit.CaseResult

pipeline {
    agent none
    parameters {
        choice(name: 'BUMP', choices: ['minor', 'patch', 'major'], description: 'What to bump when releasing') }
    options {
        buildDiscarder(logRotator(numToKeepStr: '50'))
        disableConcurrentBuilds()
    }
    environment {
        GITHUB_TOKEN = credentials('githubrelease')

        LIBRARY_NAME = 'Winit'
        REPOSITORY_OWNER = 'feenkcom'
        REPOSITORY_NAME = 'libwinit'

        MACOS_INTEL_TARGET = 'x86_64-apple-darwin'
        MACOS_M1_TARGET = 'aarch64-apple-darwin'

        WINDOWS_AMD64_SERVER_NAME = 'daffy-duck'
        WINDOWS_AMD64_TARGET = 'x86_64-pc-windows-msvc'
        WINDOWS_ARM64_SERVER_NAME = 'bugs-bunny'
        WINDOWS_ARM64_TARGET = 'aarch64-pc-windows-msvc'

        LINUX_SERVER_NAME = 'mickey-mouse'
        LINUX_AMD64_TARGET = 'x86_64-unknown-linux-gnu'
    }

    stages {
        stage ('Read tool versions') {
            agent {
                label "${MACOS_M1_TARGET}"
            }
            steps {
                script {
                    FEENK_RELEASER_VERSION = sh (
                        script: "cat feenk-releaser.version",
                        returnStdout: true
                    ).trim()
                }
                echo "Will release using feenk-releaser ${FEENK_RELEASER_VERSION}"
            }
        }
        stage ('Parallel build') {
            parallel {
                stage ('MacOS x86_64') {
                    agent {
                        label "${MACOS_INTEL_TARGET}"
                    }
                    environment {
                        TARGET = "${MACOS_INTEL_TARGET}"
                        EXTENSION = "dylib"
                        PATH = "$HOME/.cargo/bin:/usr/local/bin/:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo run --package ${REPOSITORY_NAME}-builder --bin builder --release"

                        sh "mv target/${TARGET}/release/lib${LIBRARY_NAME}.${EXTENSION} lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }
                stage ('MacOS M1') {
                    agent {
                        label "${MACOS_M1_TARGET}"
                    }

                    environment {
                        TARGET = "${MACOS_M1_TARGET}"
                        EXTENSION = "dylib"
                        PATH = "$HOME/.cargo/bin:/opt/homebrew/bin:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo run --package ${REPOSITORY_NAME}-builder --bin builder --release"

                        sh "mv target/${TARGET}/release/lib${LIBRARY_NAME}.${EXTENSION} lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }

                stage ('Linux x86_64') {
                    agent {
                        label "${LINUX_AMD64_TARGET}-${LINUX_SERVER_NAME}"
                    }
                    environment {
                        TARGET = "${LINUX_AMD64_TARGET}"
                        EXTENSION = "so"
                        PATH = "$HOME/.cargo/bin:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo run --package ${REPOSITORY_NAME}-builder --bin builder --release"

                        sh "mv target/${TARGET}/release/lib${LIBRARY_NAME}.${EXTENSION} lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }

                stage ('Windows x86_64') {
                    agent {
                        label "${WINDOWS_AMD64_TARGET}-${WINDOWS_AMD64_SERVER_NAME}"
                    }

                    environment {
                        TARGET = "${WINDOWS_AMD64_TARGET}"
                        EXTENSION = "dll"
                        CARGO_HOME = "C:\\.cargo"
                        CARGO_PATH = "${CARGO_HOME}\\bin"
                        PATH = "${CARGO_PATH};$PATH"
                    }

                    steps {
                        powershell 'git clean -fdx'

                        powershell "cargo run --package ${REPOSITORY_NAME}-builder --bin builder --release -- --target ${TARGET}"
                        powershell "Move-Item -Path target/${TARGET}/release/${LIBRARY_NAME}.${EXTENSION} -Destination ${LIBRARY_NAME}-${TARGET}.${EXTENSION}"
                        stash includes: "${LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }

                stage ('Windows arm64') {
                    agent {
                        label "${WINDOWS_ARM64_TARGET}-${WINDOWS_ARM64_SERVER_NAME}"
                    }

                    environment {
                        TARGET = "${WINDOWS_ARM64_TARGET}"
                        EXTENSION = "dll"
                        CARGO_HOME = "C:\\.cargo"
                        CARGO_PATH = "${CARGO_HOME}\\bin"
                        PATH = "${CARGO_PATH};$PATH"
                    }

                    steps {
                        powershell 'git clean -fdx'

                        powershell "cargo run --package ${REPOSITORY_NAME}-builder --bin builder --release -- --target ${TARGET}"
                        powershell "Move-Item -Path target/${TARGET}/release/${LIBRARY_NAME}.${EXTENSION} -Destination ${LIBRARY_NAME}-${TARGET}.${EXTENSION}"
                        stash includes: "${LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }
            }
        }
        stage ('Deployment') {
            agent {
                label "${MACOS_M1_TARGET}"
            }
            environment {
                TARGET = "${MACOS_M1_TARGET}"
            }
            when {
                expression {
                    (currentBuild.result == null || currentBuild.result == 'SUCCESS') && env.BRANCH_NAME.toString().equals('main')
                }
            }
            steps {
                unstash "${LINUX_AMD64_TARGET}"
                unstash "${MACOS_INTEL_TARGET}"
                unstash "${MACOS_M1_TARGET}"
                unstash "${WINDOWS_AMD64_TARGET}"
                unstash "${WINDOWS_ARM64_TARGET}"

                sh "curl -o feenk-releaser -LsS https://github.com/feenkcom/releaser-rs/releases/download/${FEENK_RELEASER_VERSION}/feenk-releaser-${TARGET}"
                sh "chmod +x feenk-releaser"

                sh """
                ./feenk-releaser \
                    --owner ${REPOSITORY_OWNER} \
                    --repo ${REPOSITORY_NAME} \
                    --token GITHUB_TOKEN \
                    --bump ${params.BUMP} \
                    --auto-accept \
                    --assets \
                        lib${LIBRARY_NAME}-${LINUX_AMD64_TARGET}.so \
                        lib${LIBRARY_NAME}-${MACOS_INTEL_TARGET}.dylib \
                        lib${LIBRARY_NAME}-${MACOS_M1_TARGET}.dylib \
                        ${LIBRARY_NAME}-${WINDOWS_AMD64_TARGET}.dll \
                        ${LIBRARY_NAME}-${WINDOWS_ARM64_TARGET}.dll """
            }
        }
    }
}
