// !!!
#[tracing::instrument]
pub async fn hi() -> &'static str {
    tracing::info!("Got hi Request.");
    "Leck Eier!"
}

#[tracing::instrument]
pub async fn hiv2() -> &'static str {
    "
    <!DOCTYPE html>
    <html>
        <body>
            <h1>Leck Eier!</h1>
        </body>
        <style>
        body {
            background-color: black;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        h1 {
            color: rgb(50, 255, 50);
        }
    </html>
    "
}
