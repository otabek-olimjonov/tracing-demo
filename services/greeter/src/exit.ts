
export const sigintHandler = () => {
    console.log('🔴 Shutting down due to SIGINT (Ctrl-C)');
    process.exit();
}