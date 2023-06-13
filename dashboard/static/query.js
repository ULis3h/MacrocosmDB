fetch('http://localhost:2475/data')
    .then(response => response.json())
    .then(data => {
        document.getElementById('data').innerText = 
            'Field1: ' + data.field1 + ', Field2: ' + data.field2;
    })
    .catch(error => console.error('Error:', error));
    document.getElementById('button1').addEventListener('click', function() {
        console.log('Button 1 clicked!');
    });
    
    document.getElementById('button2').addEventListener('click', function() {
        console.log('Button 2 clicked!');
    });
    