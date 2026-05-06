<!-- 
  📑 LABEL: LAYOUT UTAMA (app.rsx)
  Hanya menggunakan HTMX untuk interaksi.
-->
<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{% block title %}RustBasic{% endblock %}</title>
    
    <!-- 1. Google Fonts -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;800&display=swap" rel="stylesheet">
    
    <!-- 2. File CSS Utama (Injected Local) -->
    <Assets.Styles />
    
    <!-- 3. Library Frontend (HTMX Local Macro) -->
    <Assets.Htmx />
</head>
<body hx-boost="true" hx-headers='{"X-CSRF-TOKEN": "{{ csrf_token }}"}'>
    <Feedback.Indicator />
    
    <main>
        <!-- Flash Messages (Tanpa JS, dirender langsung) -->
        {% if flash_success %}
            <Display.Alert message="{{ flash_success }}" type="success" />
        {% endif %}
        {% if flash_error %}
            <Display.Alert message="{{ flash_error }}" type="error" />
        {% endif %}

        <!-- Konten dari file lain akan muncul di sini -->
        {% block content %}{% endblock %}
    </main>
</body>
</html>
