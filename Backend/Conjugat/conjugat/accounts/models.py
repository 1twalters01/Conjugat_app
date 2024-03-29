from django.db import models
from django.conf import settings

class TwoFactorAuth(models.Model):
    user = models.OneToOneField(settings.AUTH_USER_MODEL, on_delete=models.CASCADE)
    key = models.CharField(max_length=255)
    def __str__(self):
        return str(self.key)
