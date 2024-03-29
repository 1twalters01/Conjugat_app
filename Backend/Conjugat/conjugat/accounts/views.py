from django.contrib.auth import login, logout
from django.contrib.auth.models import User
from django.core.exceptions import ObjectDoesNotExist
from rest_framework import status, permissions
from rest_framework.authentication import SessionAuthentication
from rest_framework.response import Response
from rest_framework.views import APIView
from .models import *
from .serializers import *


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


class Login(APIView):
    permission_classes = (permissions.AllowAny)

    def post(self, request):
        data = {
            "email": request.data.get("email"),
            "password": request.data.get("password"),
        }
        serializer = LoginSerializer(data=data)
        if serializer.is_valid():
            try:
                user = User.objects.get(email=serializer.data["email"]);
            except ObjectDoesNotExist:
                error = "Unrecognised email"
                return Response(data=error, status=status.HTTP_404_NOT_FOUND)

            try:
                tfa_key = TwoFactorAuth.objects.get(user=user)
            except ObjectDoesNotExist:
                tfa_key = None

            if tfa_key
                # Check if tfa is correct. If not return error
            
            # Login, get a token, send token
        
        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)
    

