from django.contrib.auth import login, logout
from rest_framework import status, permissions
from rest_framework.authentication import SessionAuthentication
from rest_framework.response import Response
from rest_framework.views import APIView


class GetRoutes(APIView):
    """
    Routes
    """

    def get(self, request):
        routes = [
            {
                "Endpoint": "/login/",
                "Method": "POST",
                "Description": "Login with username and password",
            },
            {"Endpoint": "/login/totp/", "Method": "POST", "Description": "totp login"},
            {
                "Endpoint": "/login/oauth/",
                "Method": "POST",
                "Description": "oauth login",
            },
            {"Endpoint": "/logout/", "Method": "POST", "Description": "logout"},
            {
                "Endpoint": "/register/",
                "Method": "POST",
                "Description": "register a new account",
            },
            {
                "Endpoint": "/activate/",
                "Method": "POST",
                "Description": "activate an account",
            },
            {
                "Endpoint": "/password-reset/",
                "Method": "POST",
                "Description": "reset password",
            },
            {
                "Endpoint": "/password-reset/confirm/",
                "Method": "POST",
                "Description": "reset password change activation",
            },
        ]
        return Response(routes, status=status.HTTP_200_OK)
